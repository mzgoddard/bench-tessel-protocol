// process_unix_streamed.rs
// How fast can two processes write and read from each other over unix streams?

extern crate libc;

use std::iter::*;
use std::io::{Write, Read, Seek, SeekFrom};
use std::time::{Instant, Duration};
use std::thread::{yield_now, sleep};
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs::remove_file;
use std::fs::{File, OpenOptions};
use std::env::args;
use std::process::Command;

fn main() {
    println!("{:?}", args().collect::<Vec<String>>());
    let child = if args().count() == 1 {
        Some(Command::new(args().nth(0).unwrap())
          .arg("child")
          .spawn()
          .expect("failed to execute process"))
    }
    else {
        None
    };

    let mut buf = [0; 256];

    if child.is_none() {
        sleep(Duration::from_millis(1000));
        let mut s = UnixStream::connect("/tmp/bench_unixstream").unwrap();
        loop {
            s.read_exact(&mut buf).unwrap();
            s.write_all(&buf).unwrap();
        }
    }
    else {
        remove_file("/tmp/bench_unixstream");
        let mut l = UnixListener::bind("/tmp/bench_unixstream").unwrap();
        let mut s = l.accept().unwrap().0;
        let mut i = 0;
        let cid = match &child { &Some(ref c) => c.id(), &None => 0 };
        loop {
            i += 1;
            sleep(Duration::from_millis(1000));
            if i > 10 {
                break;
            }
            let start = Instant::now();
            s.write_all(&buf).unwrap();
            // unsafe { libc::kill(cid as i32, 18); }
            // yield_now();
            s.read_exact(&mut buf).unwrap();
            println!("{:?}", Instant::now().duration_since(start));
        }
        child.unwrap().kill().unwrap();
    }
}
