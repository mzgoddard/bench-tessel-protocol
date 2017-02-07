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
    let mut buf = [0; 256];

    let mut s = OpenOptions::new()
        .read(true).write(true)
        .open("/dev/ebbchar").unwrap();


    let mut i = 0;
    loop {
        i += 1;
        sleep(Duration::from_millis(1000));
        if i > 10 {
            break;
        }
        let start = Instant::now();
        for _ in 0..1000 {
            assert_eq!(s.write(&buf).unwrap(), 1);
            assert_eq!(s.read(&mut buf).unwrap(), 1);
        }
        println!("{:?}", Instant::now().duration_since(start));
    }
}
