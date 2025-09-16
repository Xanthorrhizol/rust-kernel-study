use dbus::blocking::Connection;
use dbus_crossroads::{Context, Crossroads};
use rk_lib::{Kind, World};
use std::error::Error;
use structopt::StructOpt;

struct Hello {
    called_count: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let kind = Kind::from_args();
    match kind {
        Kind::Hello => {
            let c = Connection::new_session()?;
            c.request_name("com.example.dbustest", false, true, false)?;
            let mut cr = Crossroads::new();

            let iface_token = cr.register("com.example.dbustest", |b| {
                let hello_happened = b
                    .signal::<(String,), _>("HelloHappened", ("sender",))
                    .msg_fn();
                b.method(
                    "Hello",
                    ("name",),
                    ("reply",),
                    move |ctx: &mut Context, hello: &mut Hello, (name,): (String,)| {
                        println!("클라이언트 쿼리: 안녕 {}!", name);
                        hello.called_count += 1;
                        let reply = format!("안녕 {}! API 호출 횟수: {}", name, hello.called_count);
                        let signal_msg = hello_happened(ctx.path(), &(name,));
                        ctx.push_msg(signal_msg);
                        Ok((reply,))
                    },
                );
            });
            cr.insert("/hello", &[iface_token], Hello { called_count: 0 });

            cr.serve(&c)?;
        }
        Kind::World => {
            let c = Connection::new_session()?;
            c.request_name("com.example.dbustest", false, true, false)?;
            let mut cr = Crossroads::new();

            let iface_token = cr.register("com.example.dbustest", |b| {
                let world_happened = b
                    .signal::<(String,), _>("WorldHappened", ("sender",))
                    .msg_fn();
                b.method(
                    "World",
                    ("number", "string"),
                    ("world",),
                    move |ctx: &mut Context, world: &mut World, (number, string): (i32, String)| {
                        println!("클라이언트 쿼리: number={}, string={}", number, string);
                        world.number = number;
                        world.string = string.clone();
                        let signal_msg = world_happened(ctx.path(), &(string,));
                        ctx.push_msg(signal_msg);
                        Ok((world.clone(),))
                    },
                );
            });

            cr.insert(
                "/world",
                &[iface_token],
                World {
                    number: 0,
                    string: String::default(),
                },
            );

            cr.serve(&c)?;
        }
    }

    Ok(())
}
