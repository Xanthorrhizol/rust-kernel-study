// SPDX-License-Identifier: GPL-2.0

//! A simple Rust kernel module that uses printk to log messages.

use kernel::prelude::*;

extern "C" {
    fn vprintk(fmt: *const c_char, ...) -> c_int;
}

module!(
    type: RustKernelPrintkModule,
    name: "rust_printk",
    authors: ["Rust for Linux Contributors"],
    description: "A simple Rust kernel module that uses printk",
    license: "GPL",
);

fn rust_printk(msg: &str) {
    let cstr = unsafe { CStr::from_char_ptr(msg.as_ptr() as *const c_char) };
    unsafe {
        vprintk(cstr.as_ptr());
    }
}

struct RustKernelPrintkModule;

impl kernel::Module for RustKernelPrintkModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        rust_printk("Hello from Rust kernel module!\n");
        Ok(RustKernelPrintkModule)
    }
}
