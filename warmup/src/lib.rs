use dbus::arg::{Append, AppendAll, Arg, ArgType, Get};
use std::io;
use std::os::unix::io::AsRawFd;
use structopt::StructOpt;

#[macro_export]
macro_rules! shmem {
    ($shmem_flink:expr) => {
        match shared_memory::ShmemConf::new()
            .size(4096)
            .flink($shmem_flink)
            .create()
        {
            Ok(m) => m,
            Err(shared_memory::ShmemError::LinkExists) => shared_memory::ShmemConf::new()
                .flink($shmem_flink)
                .open()
                .unwrap(),
            Err(e) => {
                panic!("공유 메모리 파일 생성 실패 {} : {e}", $shmem_flink);
            }
        }
    };
}

#[macro_export]
macro_rules! shared_data {
    ($shmem:expr) => {
        unsafe { &mut *($shmem.as_ptr() as *mut rk_lib::SharedData) }
    };
}

#[macro_export]
macro_rules! br {
    () => {
        let (_, term_width) = rk_lib::get_terminal_size().unwrap();
        br!(term_width);
    };
    ($term_width:expr) => {
        println!("{}", "=".repeat($term_width as usize));
    };
}

#[macro_export]
macro_rules! header {
    ($txt:expr) => {
        let (_, term_width) = rk_lib::get_terminal_size().unwrap();
        header!($txt, term_width);
    };
    ($txt:expr, $term_width:expr) => {
        let text = format!(" {} ", $txt);
        let padding = ($term_width as usize).saturating_sub(text.len());
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;
        println!(
            "{}{}{}",
            "=".repeat(left_padding),
            text,
            "=".repeat(right_padding)
        );
    };
}

// call get_terminal_size at compile time and set the value to TERMINAL_WIDTH as static
// does it have to use lazy_static?
// Answer: Yes, because get_terminal_size is not a const fn

#[repr(C)]
pub struct SharedData {
    pub number: i32,
}

#[derive(Debug, Clone)]
pub struct World {
    pub number: i32,
    pub string: String,
}
impl Arg for World {
    const ARG_TYPE: ArgType = ArgType::Struct;
    fn signature() -> dbus::strings::Signature<'static> {
        <(i32, String)>::signature()
    }
}
impl Append for World {
    fn append_by_ref(&self, i: &mut dbus::arg::IterAppend) {
        (self.number, self.string.clone()).append_by_ref(i);
    }
}
impl AppendAll for World {
    fn append(&self, i: &mut dbus::arg::IterAppend) {
        (self.number, self.string.clone()).append(i);
    }
}
impl<'a> Get<'a> for World {
    fn get(i: &mut dbus::arg::Iter) -> Option<Self> {
        <(i32, String)>::get(i).map(|(number, string)| World { number, string })
    }
}

#[derive(StructOpt)]
pub enum Kind {
    Hello,
    World,
}

const TIOCGWINSZ: libc::c_ulong = 0x5413;

#[repr(C)]
pub struct Winsize {
    ws_row: libc::c_ushort,
    ws_col: libc::c_ushort,
    ws_xpixel: libc::c_ushort,
    ws_ypixel: libc::c_ushort,
}

pub fn get_terminal_size() -> Result<(libc::c_ushort, libc::c_ushort), io::Error> {
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
