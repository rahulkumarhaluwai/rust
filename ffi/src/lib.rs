use std::os::raw::{c_char, c_int};
use std::panic::catch_unwind;

unsafe extern "C" {
    fn add_numbers(a: c_int, b: c_int) -> c_int;
}

#[unsafe(no_mangle)]
pub extern "C" fn task1_correct_ffi() -> c_int {
    unsafe {
        add_numbers(20, 22)
    }
}

mod wrong_ffi {
    use std::os::raw::{c_char, c_int};

    unsafe extern "C" {
        // Deliberately incorrect declaration.
        //
        // Actual C:
        //     int add_numbers(int, int)
        //
        // Rust thinks:
        //     int add_numbers(char, int)
        #[link_name = "add_numbers"]
        fn add_numbers_wrong(a: c_char, b: c_int) -> c_int;
    }

    pub unsafe fn call() -> c_int {
        unsafe {
            add_numbers_wrong(10, 20)
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn task2_wrong_ffi() -> c_int {
    unsafe {
        wrong_ffi::call()
    }
}

fn function_that_panics() -> c_int {
    panic!("Rust function panicked!");
}

#[unsafe(no_mangle)]
pub extern "C" fn task3_safe_panic_wrapper() -> c_int {
    match catch_unwind(function_that_panics) {
        Ok(value) => value,
        Err(_) => -1,
    }
}