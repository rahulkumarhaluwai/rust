use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    race_vs_mutex();
    mutex_poisoning();
    rwlock_readers_and_writer();
}

struct UnsafeCounter {
    value: UnsafeCell<i32>,
}

// Deliberately unsafe.
//
// We are telling Rust that UnsafeCounter can be shared between
// threads, even though access to the contained i32 is unsynchronized.
//
// This is ONLY for demonstrating what goes wrong without synchronization.
unsafe impl Sync for UnsafeCounter {}
unsafe impl Send for UnsafeCounter {}

fn race_vs_mutex() {
    const THREADS: usize = 8;
    const INCREMENTS: usize = 1_000;
    
    println!("\n--- Unsynchronized counter ---");

    let counter = Arc::new(UnsafeCounter {
        value: UnsafeCell::new(0),
    });

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let counter = Arc::clone(&counter);

        handles.push(thread::spawn(move || {
            for _ in 0..INCREMENTS {
                unsafe {
                    // Read
                    let current = *counter.value.get();

                    // Force another thread to potentially run here.
                    thread::yield_now();

                    // Write
                    *counter.value.get() = current + 1;
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let expected = THREADS * INCREMENTS;

    let actual = unsafe { *counter.value.get() };

    println!("expected = {expected}");
    println!("actual   = {actual}");

    if actual == expected as i32 {
        println!("Result happened to be correct this run.");
    } else {
        println!("WRONG COUNT! Lost updates occurred.");
    }

    println!("Note: this code has undefined behavior because the");
    println!("shared integer is accessed without synchronization.");

    println!("\n--- Arc<Mutex<i32>> ---");

    let counter = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..THREADS {
        let counter = Arc::clone(&counter);

        handles.push(thread::spawn(move || {
            for _ in 0..INCREMENTS {
                let mut value = counter.lock().unwrap();

                // Keep the forced interleaving.
                //
                // Another thread may run here, but it cannot
                // enter this critical section because we hold
                // the mutex.
                thread::yield_now();

                *value += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let expected = THREADS * INCREMENTS;
    let actual = *counter.lock().unwrap();

    println!("expected = {expected}");
    println!("actual   = {actual}");

    assert_eq!(actual, expected as i32);

    println!("Mutex result is exactly correct.");
}

fn mutex_poisoning() {
    let data = Arc::new(Mutex::new(42));

    let data_for_thread = Arc::clone(&data);

    let handle = thread::spawn(move || {
        let mut value = data_for_thread.lock().unwrap();

        println!("worker: acquired mutex");
        println!("worker: value before = {value}");

        *value = 100;

        println!("worker: value changed to {value}");
        println!("worker: panicking while holding the mutex!");

        panic!("intentional panic for poisoning experiment");
    });

    // The worker thread panics.
    let result = handle.join();

    println!("worker result: {:?}", result);

    // Because the worker panicked while holding the mutex,
    // the mutex is now poisoned.
    match data.lock() {
        Ok(value) => {
            println!("mutex was not poisoned");
            println!("value = {value}");
        }

        Err(poisoned) => {
            println!("mutex is poisoned!");

            // Recover the MutexGuard instead of calling unwrap()
            // and crashing the main thread.
            let value = poisoned.into_inner();

            println!("recovered value = {value}");

            assert_eq!(*value, 100);

            println!("Successfully recovered the data.\n");
        }
    }
}

fn rwlock_readers_and_writer() {
    let map = Arc::new(RwLock::new(HashMap::<String, i32>::new()));

    let start = Instant::now();

    let mut handles = Vec::new();

    // --------------------------------------------------------
    // Three reader threads
    // --------------------------------------------------------

    for id in 0..3 {
        let map = Arc::clone(&map);

        handles.push(thread::spawn(move || {
            let guard = map.read().unwrap();

            let begin = start.elapsed();

            println!(
                "reader {id} START  at {:>4} ms",
                begin.as_millis()
            );

            // Hold the read lock for a while.
            //
            // If RwLock permits concurrent readers, the other
            // readers should also be able to enter during this
            // sleep.
            thread::sleep(Duration::from_millis(500));

            let end = start.elapsed();

            println!(
                "reader {id} END    at {:>4} ms",
                end.as_millis()
            );

            drop(guard);
        }));
    }

    // --------------------------------------------------------
    // One writer thread
    // --------------------------------------------------------

    {
        let map = Arc::clone(&map);

        handles.push(thread::spawn(move || {
            // Give the readers time to acquire their locks.
            thread::sleep(Duration::from_millis(100));

            let waiting = start.elapsed();

            println!(
                "writer     WAIT   at {:>4} ms",
                waiting.as_millis()
            );

            // This cannot succeed while any reader is holding
            // a read lock.
            let mut guard = map.write().unwrap();

            let acquired = start.elapsed();

            println!(
                "writer     START  at {:>4} ms",
                acquired.as_millis()
            );

            guard.insert("answer".to_string(), 42);

            // Keep the write lock for a while so it is obvious
            // that readers cannot enter during this period.
            thread::sleep(Duration::from_millis(300));

            let end = start.elapsed();

            println!(
                "writer     END    at {:>4} ms",
                end.as_millis()
            );

            drop(guard);
        }));
    }

    // Wait for every thread.
    for handle in handles {
        handle.join().unwrap();
    }

    // --------------------------------------------------------
    // Verify final data
    // --------------------------------------------------------

    let final_map = map.read().unwrap();

    println!("\nfinal map = {:?}", *final_map);

    assert_eq!(final_map.get("answer"), Some(&42));

    println!("Final map contains the expected value.");
}