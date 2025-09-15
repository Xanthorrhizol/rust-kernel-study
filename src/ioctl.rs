extern crate libc;

use libc::{STDOUT_FILENO, write};
use std::ffi::CString;
use std::io;
use std::os::unix::io::AsRawFd;

const TIOCGWINSZ: libc::c_ulong = 0x5413;

#[repr(C)]
struct Winsize {
    ws_row: libc::c_ushort,
    ws_col: libc::c_ushort,
    ws_xpixel: libc::c_ushort,
    ws_ypixel: libc::c_ushort,
}

fn get_terminal_size() -> Result<(libc::c_ushort, libc::c_ushort), io::Error> {
    let ws = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let fd = io::stdout().as_raw_fd();

    let result = unsafe { libc::ioctl(fd, TIOCGWINSZ, &ws) };

    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok((ws.ws_row, ws.ws_col))
    }
}

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
