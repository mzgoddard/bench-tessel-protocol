// baseline_unixstream.rs
// How fast can a stream pair in one process be written into and read from?

extern crate libc;

use std::iter::*;
use std::io::{Write, Read};
use std::time::{Instant, Duration};
use std::thread::{yield_now, sleep};
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs::remove_file;
use std::env::args;
use std::process::Command;

fn main() {
    let (mut a, mut b) = UnixStream::pair().unwrap();
    let mut buf = [0; 256];
    for _ in 0..10 {
        sleep(Duration::from_millis(100));
        let start = Instant::now();
        for _ in 0..1000 {
            a.write_all(&buf).unwrap();
            b.read_exact(&mut buf).unwrap();
        }
        println!("{:?}", Instant::now().duration_since(start));
    }
}
