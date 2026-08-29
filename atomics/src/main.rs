use std::cell::UnsafeCell;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

// ============================================================
// SpinLock
// ============================================================

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    // ========================================================
    // Normal lock()
    //
    // compare_exchange_weak makes sense here because we are
    // going to retry anyway if the operation fails.
    // ========================================================

    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        loop {
            match self.locked.compare_exchange_weak(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    return SpinLockGuard { lock: self };
                }
                Err(_) => {
                    std::hint::spin_loop();
                }
            }
        }
    }

    // ========================================================
    // try_lock()
    //
    // Exactly ONE compare_exchange attempt.
    //
    // Strong compare_exchange is used because a failure must
    // mean that the lock was actually unavailable, rather than
    // allowing a spurious failure.
    // ========================================================

    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        match self.locked.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            Ok(_) => Some(SpinLockGuard { lock: self }),
            Err(_) => None,
        }
    }
}

impl<T> std::ops::Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> std::ops::DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

// ============================================================
// PART 1
// Relaxed vs Acquire/Release
// ============================================================
//
// 50 threads × 10,000 increments = 500,000 expected.
//
// IMPORTANT:
// Seeing 500,000 with Relaxed does NOT prove that Relaxed is
// correct for synchronization. It only means this particular
// experiment did not expose a failure.
//
// The lock itself still provides mutual exclusion. The purpose
// here is to compare the memory ordering used by the lock.
// ============================================================

const THREADS: usize = 50;
const ITERS: usize = 10_000;

fn run_counter_test(acquire_release: bool) -> usize {
    let lock = Arc::new(SpinLock::new(0usize));

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let lock = Arc::clone(&lock);

        let handle = thread::spawn(move || {
            for _ in 0..ITERS {
                if acquire_release {
                    // Normal Acquire/Release SpinLock behavior.
                    let mut guard = lock.lock();
                    *guard += 1;
                } else {
                    // This section intentionally demonstrates the
                    // Relaxed version of the atomic lock operation.
                    //
                    // We access the AtomicBool directly here so that
                    // the same SpinLock can be tested with Relaxed.
                    loop {
                        match lock.locked.compare_exchange_weak(
                            false,
                            true,
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                        ) {
                            Ok(_) => break,
                            Err(_) => std::hint::spin_loop(),
                        }
                    }

                    unsafe {
                        *lock.data.get() += 1;
                    }

                    lock.locked.store(false, Ordering::Relaxed);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe { *lock.data.get() }
}

fn relaxed_vs_acquire_release_test() {
    let expected = THREADS * ITERS;
    println!("threads   = {}", THREADS);
    println!("iterations = {}", ITERS);
    println!("expected  = {}", expected);
    println!();

    let relaxed = run_counter_test(false);

    println!("--- Relaxed ---");
    println!("expected = {}", expected);
    println!("actual   = {}", relaxed);

    if relaxed == expected {
        println!("Relaxed produced the expected result on this run.");
    } else {
        println!("RELAXED FAILURE: incorrect count!");
    }

    println!();

    let acquire_release = run_counter_test(true);

    println!("--- Acquire/Release ---");
    println!("expected = {}", expected);
    println!("actual   = {}", acquire_release);

    if acquire_release == expected {
        println!("Acquire/Release result is correct.");
    } else {
        println!("Acquire/Release FAILURE!");
    }

    println!();
}

// ============================================================
// PART 2
// try_lock()
// ============================================================
//
// We prove:
//
// 1. Another thread acquires the lock.
// 2. While it is held, try_lock() returns None.
// 3. The other thread releases the lock.
// 4. try_lock() then returns Some.
// ============================================================

fn try_lock_test() {
    let lock = Arc::new(SpinLock::new(42usize));

    // Used to tell the main thread that the worker has
    // successfully acquired the lock.
    let acquired = Arc::new(AtomicBool::new(false));

    let lock_for_thread = Arc::clone(&lock);
    let acquired_for_thread = Arc::clone(&acquired);

    let handle = thread::spawn(move || {
        let _guard = lock_for_thread.lock();

        // Tell main thread that we definitely hold the lock.
        acquired_for_thread.store(true, Ordering::Release);

        // Keep the lock held for a short time.
        thread::sleep(Duration::from_millis(200));

        // _guard is dropped here.
        // That releases the lock.
    });

    // Wait until worker has acquired the lock.
    while !acquired.load(Ordering::Acquire) {
        thread::yield_now();
    }

    // The worker currently holds the lock.
    match lock.try_lock() {
        Some(_) => {
            println!("ERROR: try_lock() returned Some while lock was held.");
        }
        None => {
            println!("try_lock() while locked -> None");
        }
    }

    // Wait for worker to release the lock.
    handle.join().unwrap();

    // Now the lock should be available.
    match lock.try_lock() {
        Some(guard) => {
            println!("try_lock() after release -> Some");
            println!("value = {}", *guard);
        }
        None => {
            println!("ERROR: try_lock() returned None after lock was released.");
        }
    }

    println!();
}

// ============================================================
// PART 3
// Why weak vs strong compare_exchange?
// ============================================================
//
// compare_exchange_weak:
//   - May fail spuriously.
//   - Therefore it is appropriate when we are going to retry.
//   - A spin lock naturally retries.
//
// compare_exchange:
//   - Does not have spurious failure.
//   - A failure means the comparison genuinely failed.
//   - Therefore it is appropriate for one-shot try_lock().
// ============================================================

fn compare_exchange_explanation() {
    println!("compare_exchange_weak:");
    println!("  - Can fail spuriously.");
    println!("  - Makes sense inside a retry/spin loop.");
    println!("  - A spurious failure is harmless because we try again.");
    println!();

    println!("compare_exchange:");
    println!("  - Does not allow spurious failure.");
    println!("  - Makes sense for a one-shot operation.");
    println!("  - try_lock() needs exactly this behavior.");
    println!();

    println!("In this project:");
    println!("  lock()     -> compare_exchange_weak()");
    println!("  try_lock() -> compare_exchange()");
    println!();
}

fn main() {
    relaxed_vs_acquire_release_test();

    try_lock_test();

    compare_exchange_explanation();
}