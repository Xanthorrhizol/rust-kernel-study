extern crate shared_memory;

use rk_lib::SharedData;
use rk_lib::{shared_data, shmem};
use std::thread;
use std::time::Duration;

fn main() {
    let shmem_flink = "/tmp/basic_mapping.map";

    let shmem = shmem!(shmem_flink);
    let shared_data: &mut SharedData = shared_data!(shmem);

    shared_data.number = 0;

    while shared_data.number < 60 {
        println!("Writing: {}", shared_data.number);
        shared_data.number += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
