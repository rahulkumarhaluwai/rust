use std::thread;

fn main() {
    let mut handles = Vec::new();

    handles.push(thread::spawn(|| {
        println!("Hello from thread 1");
    }));

    handles.push(thread::spawn(|| {
        println!("Hello from thread 2");
    }));

    handles.push(thread::spawn(|| {
        println!("Hello from thread 3");
    }));

    // All three messages must print, but order may vary.
    for handle in handles {
        handle.join().unwrap();
    }

    struct RawWrapper {
        ptr: *mut i32,
    }

    unsafe impl Send for RawWrapper {}

    struct Thing {
        raw: RawWrapper,
        value: i32,
    }

    let thing = Thing {
        raw: RawWrapper {
            ptr: std::ptr::null_mut(),
        },
        value: 42,
    };

    // Only `thing.value` is captured, not the RawWrapper field.
    // Therefore no `let thing = thing;` rebind is needed.
    thread::spawn(move || {
        println!("Non-pointer field: {}", thing.value);
    })
    .join()
    .unwrap();
}