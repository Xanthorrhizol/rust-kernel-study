use memmap::MmapOptions;
use std::fs::OpenOptions;
use structopt::StructOpt;

const FILE_PATH: &str = "shared_mmap_file.txt";

#[derive(Debug, StructOpt)]
enum Opts {
    Read,
    Write,
}

fn main() {
    let opts = Opts::from_args();
    match opts {
        Opts::Read => {
            println!("Reading from mmap...");
            read_from_mmap();
        }
        Opts::Write => {
            println!("Writing to mmap...");
            write_to_mmap();
        }
    }
}

fn write_to_mmap() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();

    let message = b"Hello from mmap!";
    file.set_len(message.len() as u64).unwrap();

    let mut mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };

    mmap.copy_from_slice(message);

    println!("메시지를 작성했습니다.");
}

fn read_from_mmap() {
    let file = OpenOptions::new().read(true).open(FILE_PATH).unwrap();

    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

    let content = String::from_utf8_lossy(&mmap);
    println!("Read from mmap: {}", content);
}
