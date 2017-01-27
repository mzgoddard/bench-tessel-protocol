// process_stdio_pipes.rs
// How fast can two processes write and read from each other over pipes?

use std::iter::*;
use std::io::{Read, Write};
use std::io;
use std::time::{Instant, Duration};
use std::thread::{yield_now, sleep};
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs::remove_file;
use std::env::args;
use std::process::{Command, Stdio};

fn main() {
    let child = if args().count() == 1 {
        println!("{:?}", args().collect::<Vec<String>>());
        Some(Command::new(args().nth(0).unwrap())
          .arg("child")
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          .spawn()
          .expect("failed to execute process"))
    }
    else {
        None
    };

    let mut buf = [0; 255];

    if let Some(mut child) = child {
        // remove_file("/tmp/bench_unixstream");
        // let mut l = UnixListener::bind("/tmp/bench_unixstream").unwrap();
        // let mut s = l.accept().unwrap().0;
        let mut childin = child.stdin.take().unwrap();
        let mut childout = child.stdout.take().unwrap();
        let start = Instant::now();
        childin.write_all(&buf).unwrap();
        childin.flush().unwrap();
        println!("wrote to child");
        childout.read_exact(&mut buf).unwrap();
        println!("{:?}", Instant::now().duration_since(start));
        childin.write_all(&buf).unwrap();
        childin.flush().unwrap();
        println!("wrote to child");
        childout.read_exact(&mut buf).unwrap();
        println!("{:?}", Instant::now().duration_since(start));
    }
    else {
        // sleep(Duration::from_millis(1000));
        // let mut s = UnixStream::connect("/tmp/bench_unixstream").unwrap();
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        stdin.read_exact(&mut buf).unwrap();
        stdout.write_all(&buf).unwrap();
        stdout.flush().unwrap();
        stdin.read_exact(&mut buf).unwrap();
        stdout.write_all(&buf).unwrap();
        stdout.flush().unwrap();
    }
}
