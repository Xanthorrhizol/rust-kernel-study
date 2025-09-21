extern crate ipc_channel;
extern crate libc;
extern crate nix;

use ipc_channel::ipc::{self, IpcReceiver, IpcSender};
use libc::{
    sched_get_priority_max, sched_get_priority_min, sched_getscheduler, sched_param,
    sched_setscheduler,
};
use nix::sys::wait::{WaitStatus, wait, waitpid};
use nix::unistd::{ForkResult, close, fork, getpid, pipe, read, write};
use rk_lib::header;
use std::str;
use std::thread;
use std::time::Duration;
use std::{process, process::Command};

fn main() {
    let self_pid = std::process::id();
    {
        header!("process");
        let mut child_process = Command::new("sleep")
            .arg("3")
            .spawn()
            .expect("Failed to spawn child process");
        let pid = child_process.id();

        println!("Self PID: {}, Child PID: {}", self_pid, pid);

        let status = child_process
            .wait()
            .expect("Failed to wait for child prfocess to complete");

        println!("Child process exited with status: {:?}", status);
    }
    {
        header!("thread");
        let handle = thread::spawn(|| {
            let self_pid = std::process::id();
            println!("Thread PID: {}", self_pid);
        });
        println!("Thread started, thread ID: {:?}", handle.thread().id());
        handle.join().expect("Thread panicked");
        println!("Thread completed.");
    }
    {
        header!("fork");
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                println!("In parent process. Child PID: {}", child);
                match waitpid(child, None) {
                    Ok(WaitStatus::Exited(pid, status)) => {
                        println!("Child process {} exited with status: {}", pid, status);
                    }
                    Ok(WaitStatus::Signaled(pid, signal, _)) => {
                        println!("Child process {} was killed by signal: {}", pid, signal);
                    }
                    Err(e) => {
                        eprintln!("Failed to wait for child process: {}", e);
                        process::exit(1);
                    }
                    _ => {}
                }
            }
            Ok(ForkResult::Child) => {
                println!("In child process. PID: {}", std::process::id());
                thread::sleep(Duration::from_secs(2));
                println!("Child process exiting.");
                process::exit(0);
            }
            Err(e) => {
                eprintln!("Fork failed: {}", e);
                process::exit(1);
            }
        }
    }
    {
        header!("schduling");
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                let max_priority = unsafe { sched_get_priority_max(libc::SCHED_RR) };
                let min_priority = unsafe { sched_get_priority_min(libc::SCHED_RR) };
                let priority = (min_priority + max_priority) / 2;
                let mut param = sched_param {
                    sched_priority: priority,
                };
                let result =
                    unsafe { sched_setscheduler(child.into(), libc::SCHED_RR, &mut param) };
                if result == -1 {
                    eprintln!(
                        "Failed to set scheduling policy: {}",
                        std::io::Error::last_os_error()
                    );
                    process::exit(1);
                }
                waitpid(child, None).expect("Failed to wait for child");
            }
            Ok(ForkResult::Child) => {
                let pid = getpid();
                thread::sleep(Duration::from_secs(1));
                let scheduling_policy = unsafe { sched_getscheduler(pid.into()) };

                println!(
                    "Child PID: {}, Scheduling Policy: {}",
                    pid, scheduling_policy
                );

                thread::sleep(Duration::from_secs(5));
                println!("Child process exiting.");
                process::exit(0);
            }
            Err(e) => {
                eprintln!("Fork failed: {}", e);
                process::exit(1);
            }
        }
    }
    {
        header!("pipe");
        let (read_fd, write_fd) = pipe().expect("Failed to create pipe");
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                close(read_fd).expect("Failed to close read end in parent");
                let message = "Hello from parent";
                write(&write_fd, message.as_bytes()).expect("Failed to write to pipe");
                close(write_fd).expect("Failed to close write end in parent");
                waitpid(child, None).expect("Failed to wait for child");
            }
            Ok(ForkResult::Child) => {
                close(write_fd).expect("Failed to close write end in child");
                let mut buf = [0u8; 1024];
                let nbytes = read(&read_fd, &mut buf).expect("Failed to read from pipe");
                close(read_fd).expect("Failed to close read end in child");
                let received_message = str::from_utf8(&buf[..nbytes]).expect("Invalid UTF-8");
                println!("Child received message: {}", received_message);
                process::exit(0);
            }
            Err(e) => {
                eprintln!("Fork failed: {}", e);
                process::exit(1);
            }
        }
    }
    {
        header!("ipc-channel");
        let (tx, rx): (IpcSender<String>, IpcReceiver<String>) =
            ipc::channel().expect("IPC channel creation failed");

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                let message = "Hello from parent via IPC";
                tx.send(message.to_string())
                    .expect("Failed to send message via IPC");
                waitpid(child, None).expect("Failed to wait for child");
            }
            Ok(ForkResult::Child) => {
                let received_message = rx.recv().expect("Failed to receive message via IPC");
                println!("Child received message: {}", received_message);
                process::exit(0);
            }
            Err(e) => {
                eprintln!("Fork failed: {}", e);
                process::exit(1);
            }
        }
    }
}
