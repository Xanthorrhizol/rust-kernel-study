extern crate libc;

use libc::{STDOUT_FILENO, write};
use rk_lib::get_terminal_size;
use std::ffi::CString;

fn main() {
    let (rows, cols) = match get_terminal_size() {
        Ok((rows, cols)) => {
            println!("Terminal size: {} rows, {} columns", rows, cols);
            (rows, cols)
        }
        Err(e) => {
            panic!("Error getting terminal size: {}", e);
        }
    };
    println!("Oh, let's doing something fun with it!");
    let s = unsafe { CString::from_vec_unchecked(vec![b'#'; cols as usize]) };
    for _ in 0..rows {
        unsafe {
            write(STDOUT_FILENO, s.as_ptr() as *const _, s.as_bytes().len());
        }
    }
    println!("뭣하러 굳이 이렇게?");
    let v = vec![b'#'; cols as usize];
    let s = String::from_utf8_lossy(&v);
    for _ in 0..rows {
        println!("{}", s);
    }
}
