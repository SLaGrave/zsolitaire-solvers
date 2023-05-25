extern crate libc;

use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
#[warn(improper_ctypes_definitions)]  // String not FFI safe
pub extern "C" fn custom_print(s: String, b: bool) {
    if b {
        println!("From Rust: {}", s);
    }
}

#[no_mangle]
pub extern "C" fn run_simulation(
    col_a: *const c_char,
    col_b: *const c_char,
    col_c: *const c_char,
    col_d: *const c_char
) -> bool {

    let a = unsafe {
        CStr::from_ptr(col_a)
    }.to_str().unwrap();
    let b = unsafe {
        CStr::from_ptr(col_b)
    }.to_str().unwrap();
    let c = unsafe {
        CStr::from_ptr(col_c)
    }.to_str().unwrap();
    let d = unsafe {
        CStr::from_ptr(col_d)
    }.to_str().unwrap();

    let a_vec = make_vec(a.to_string());
    let b_vec = make_vec(b.to_string());
    let c_vec = make_vec(c.to_string());
    let d_vec = make_vec(d.to_string());

    

    true
}

fn calc(a: Vec<u32>, b: Vec<u32>, c: Vec<u32>, d: Vec<u32>) -> (i32, String) {
    (0, "test".to_string())
}

fn make_vec(s: String) -> Vec<u32> {
    let mut v = vec!();
    for c in s.chars() {
        let i = match c {
            'A' => 1,
            'J' => 10,
            'Q' => 10,
            'K' => 10,
            _ => c.to_digit(10).unwrap()
        };
        v.push(i);
    }
    v
}