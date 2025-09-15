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
