use autocxx::prelude::*;
use std::mem;

include_cpp! {
    #include "input.h"
    safety!(unsafe_ffi)
    generate!("x3")
    generate!("Test")
}

fn main() {
    {
        println!("###### print ######");
        println!("4x3={}", ffi::x3(4));

        let mut test = ffi::Test::new().within_box();

        test.as_mut().inc();
        test.as_mut().inc();

        println!("{}", test.to_string().as_ref().unwrap().to_string_lossy());
    }

    {
        println!("###### memory ######");
        let src = [1, 2, 3];
        let mut dest = [0; 3];

        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), dest.as_mut_ptr(), src.len());
        }

        println!("{:?}", dest);
    }

    {
        println!("###### malloc ######");
        let message = "Hello Rust\0".as_ptr() as *const libc::c_char;
        unsafe {
            let ptr: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;

            if ptr.is_null() {
                panic!("메모리 할당 실패");
            }

            let val: *mut i32 = ptr;

            *val = 123;

            println!("*ptr={}", *ptr);

            libc::free(ptr as *mut libc::c_void);

            libc::printf(message);
        }
    }
}
