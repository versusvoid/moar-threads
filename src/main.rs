extern crate rand;
extern crate rlimit;

use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;

fn client(port: u16) {
    let mut stream = TcpStream::connect(("127.0.0.1", port))
        .expect(&format!("Can't connect to port {}", port));
    stream.set_nonblocking(false).expect("can't set nonblocking client");
    let data: &[u8; 4] = b"ping";
    loop {
        if let Err(e) = stream.write(data) {
            println!("Error writing stream: {e:?}");
        }

        let ms = (rand::random::<f64>() * 1000.) as u64;
        thread::sleep(Duration::from_millis(ms));
    }
}

fn server(mut stream: TcpStream) {
    stream.set_nonblocking(false).expect("can't set nonblocking server");
    let mut data = [0u8; 16];
    loop {
        let n = match stream.read(&mut data) {
            Ok(n) => n as i64,
            Err(e) => {
                println!("Error reading from stream: {e:?}");
                -1
            },
        };
        assert_ne!(n, 0);
    }
}

fn main() {
    println!("nofile limit: {:?}", rlimit::increase_nofile_limit(u64::MAX));
    let mut port = 8080;
    let mut listener = TcpListener::bind(("127.0.0.1", port))
        .expect(&format!("can't listen on port {}", port));
    for i in 1..=100_000 {
        if i % 25_000 == 0 {
            port += 1;
            listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
        }

        if let Err(e) = thread::Builder::new().spawn(move || client(port)) {
            println!("Can't spawn client i={i}: {e:?}");
            return;
        }

        let socket = match listener.accept() {
            Ok((socket, _)) => socket,
            Err(e) => {
                println!("can't get client: {e:?}");
                return;
            },
        };

        if let Err(e) = thread::Builder::new().spawn(move || server(socket)) {
            println!("Can't spawn server i={i}: {e:?}");
            return;
        }

        if i % 1000 == 0 {
            println!("{i}");
        }
    }

    thread::sleep(Duration::from_secs(120));
    println!("Done");
}
