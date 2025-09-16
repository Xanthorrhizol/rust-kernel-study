use dbus::blocking::Connection;
use rk_lib::{Kind, World};
use std::time::Duration;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kind = Kind::from_args();
    match kind {
        Kind::Hello => {
            let conn = Connection::new_session()?;
            let proxy = conn.with_proxy(
                "com.example.dbustest",
                "/hello",
                Duration::from_millis(5000),
            );
            let (hello,): (String,) =
                proxy.method_call("com.example.dbustest", "Hello", ("luna",))?;

            println!("수신: {}", hello);
        }
        Kind::World => {
            let mut buf = String::new();
            println!("숫자 입력: ");
            std::io::stdin().read_line(&mut buf).unwrap();
            let number: i32 = buf.trim().parse().unwrap();
            let mut string = String::new();
            println!("문자열 입력: ");
            std::io::stdin().read_line(&mut string).unwrap();
            string = string.trim().to_string();

            let conn = Connection::new_session()?;
            let proxy = conn.with_proxy(
                "com.example.dbustest",
                "/world",
                Duration::from_millis(5000),
            );
            let (world,): (World,) =
                proxy.method_call("com.example.dbustest", "World", (number, string))?;

            println!("수신: {:?}", world);
        }
    }

    Ok(())
}
