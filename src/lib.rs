use dbus::arg::{Append, AppendAll, Arg, ArgType, Get};
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
