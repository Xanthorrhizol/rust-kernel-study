use signal_hook::{consts::SIGINT, iterator::Signals};
use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Error> {
    let signal_received = Arc::new(AtomicBool::new(false));
    let signal_received_clone = signal_received.clone();

    signal_hook::flag::register(SIGINT, signal_received_clone)?;

    println!("SIGINT를 수신하거나 Ctrl+C를 입력하면 프로그램을 종료합니다.");

    while !signal_received.load(Ordering::Relaxed) {
        sleep(Duration::from_secs(1));
    }

    println!("SIGINT 신호를 수신했습니다. 프로그램을 종료... 하기 전에 다른거 더 해보겠음.");

    println!("시그널 날려보세요");

    let mut signals = Signals::new(&[SIGINT])?;

    std::thread::spawn(move || {
        for sig in signals.forever() {
            println!("SIGNAL 수신: {sig}");
            std::process::exit(0);
        }
    });

    sleep(Duration::from_secs(10));

    Ok(())
}
