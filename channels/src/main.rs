// main.rs
//
// Three self-contained demos of std::sync::mpsc channels:
//   1. A three-stage pipeline (generate -> square -> collect), no shared
//      mutable state anywhere - just ownership moving through channels.
//   2. sync_channel backpressure, timestamped, comparing bound = 0 vs bound = 5.
//   3. Why dropping every Sender lets a `for` loop over a Receiver end on
//      its own instead of blocking forever.
//
// Run with:  rustc main.rs -o pipeline && ./pipeline
// (or drop it in a `cargo new` project as src/main.rs)

use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    section("PART 1: three-stage pipeline (generate -> square -> collect)");
    pipeline_demo();

    section("PART 2a: sync_channel(0) - rendezvous backpressure");
    backpressure_demo(0);

    section("PART 2b: sync_channel(5) - buffered backpressure");
    backpressure_demo(5);

    section("PART 3: dropping every Sender ends the Receiver's for-loop");
    drop_demo();
}

fn section(title: &str) {
    println!("\n=== {title} ===");
}

// ---------------------------------------------------------------------
// PART 1: pipeline
// ---------------------------------------------------------------------
//
// Stage 1 (generator thread) owns tx1 and only ever sends.
// Stage 2 (squarer thread)   owns rx1 and tx2 - it receives, transforms,
//                            and re-sends. It never touches stage 1's or
//                            stage 3's data concurrently; ownership of each
//                            value simply moves down the pipe.
// Stage 3 (main thread)      owns rx2 and only ever receives.
//
// No Mutex, no Arc<Mutex<_>>, no shared mutable state at all - each value
// is exclusively owned by exactly one thread at any given time.

fn pipeline_demo() {
    let (num_tx, num_rx) = mpsc::channel::<u64>();
    let (sq_tx, sq_rx) = mpsc::channel::<u64>();

    // Stage 1: generate 0..20
    let generator = thread::spawn(move || {
        for n in 0..20u64 {
            num_tx.send(n).expect("stage 2 hung up early");
        }
        // num_tx is dropped here (end of closure) -> channel 1 closes
    });

    // Stage 2: receive, square, forward
    let squarer = thread::spawn(move || {
        for n in num_rx {
            // this `for` ends once num_tx is dropped AND the buffer is drained
            sq_tx.send(n * n).expect("main hung up early");
        }
        // sq_tx is dropped here -> channel 2 closes
    });

    // Stage 3: collect on main thread
    let results: Vec<u64> = sq_rx.iter().collect();

    generator.join().unwrap();
    squarer.join().unwrap();

    println!("collected {} results:", results.len());
    println!("{results:?}");
}

// ---------------------------------------------------------------------
// PART 2: sync_channel backpressure
// ---------------------------------------------------------------------
//
// sync_channel(bound) gives the channel a fixed-capacity buffer.
// `send` only blocks once that buffer is full and there's nowhere to put
// the value. bound = 0 means the buffer holds zero items, so every single
// send has to rendezvous directly with a matching recv on the other side.

const ITEMS: u64 = 8;
const RECEIVER_DELAY_MS: u64 = 200;

fn backpressure_demo(bound: usize) {
    let (tx, rx): (SyncSender<u64>, Receiver<u64>) = mpsc::sync_channel(bound);
    let start = Instant::now();

    let sender = thread::spawn(move || {
        for i in 0..ITEMS {
            let t0 = start.elapsed();
            tx.send(i).unwrap();
            let blocked_for = start.elapsed() - t0;
            println!(
                "  [t={:>8.2?}] sender:   send({i}) returned   (blocked {blocked_for:>8.2?})",
                start.elapsed()
            );
        }
    });

    for _ in 0..ITEMS {
        thread::sleep(Duration::from_millis(RECEIVER_DELAY_MS));
        let v = rx.recv().unwrap();
        println!("  [t={:>8.2?}] receiver: recv() -> {v}", start.elapsed());
    }

    sender.join().unwrap();
}

// ---------------------------------------------------------------------
// PART 3: every Sender dropped -> Receiver's for-loop ends
// ---------------------------------------------------------------------
//
// Three worker threads each hold a *clone* of the same Sender. Cloning a
// Sender bumps an internal count; dropping one decrements it. The
// original Sender is dropped explicitly right after spawning. Once every
// last clone (the 3 worker clones + the original) is gone, the channel
// is marked disconnected and the `for` loop over the Receiver ends on
// its own - see the write-up for why.

fn drop_demo() {
    let (tx, rx) = mpsc::channel::<String>();

    let mut workers = Vec::new();
    for id in 0..3 {
        let worker_tx = tx.clone(); // sender count + 1
        workers.push(thread::spawn(move || {
            worker_tx.send(format!("hello from worker {id}")).unwrap();
            // worker_tx dropped here at end of closure -> sender count - 1
        }));
    }

    // Drop our own copy explicitly. If we forgot this line, one Sender
    // would still be alive (owned by `main`/this function) for as long as
    // the program runs, the count would never hit zero, and the loop
    // below would block forever after printing the three messages.
    drop(tx);

    for msg in rx {
        println!("  collector received: {msg}");
    }
    println!("  for-loop over rx ended on its own - no Senders remain");

    for w in workers {
        w.join().unwrap();
    }
}