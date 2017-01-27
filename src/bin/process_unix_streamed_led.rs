// process_unix_streamed.rs
// How fast can two processes write and read from each other over unix streams
// while the "remote" process reads and writes to a sysfs led?
//
// A goal here is to consider the timing of entering the kernel to read and
// write to the sysfs driver for a led. This can hopefully help indicate the
// time spid takes reading the irq gpio and writing the sync gpio pins.

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
        let mut oo = OpenOptions::new();
        oo.read(true).write(true).create(true);
        let mut f = oo.open("/sys/devices/leds/leds/tessel:blue:user2/brightness").unwrap();
        sleep(Duration::from_millis(1000));
        let mut s = UnixStream::connect("/tmp/bench_unixstream").unwrap();
        loop {
            s.read_exact(&mut buf).unwrap();
            let mut fb = [49, 0];
            f.seek(SeekFrom::Start(0)).unwrap();
            f.read_exact(&mut fb).is_ok();
            fb[0] = 49;
            f.write_all(&fb).unwrap();
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
