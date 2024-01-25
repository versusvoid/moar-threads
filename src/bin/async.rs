use std::time::Duration;

use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

fn main() {
    println!("nofile limit: {:?}", rlimit::increase_nofile_limit(u64::MAX));
    task::block_on(main_async());
}

async fn client(mut stream: TcpStream) {
    let data: &[u8; 4] = b"ping";
    loop {
        if let Err(e) = stream.write(data).await {
            println!("Error writing stream: {e:?}");
        }

        let ms = (rand::random::<f64>() * 1000.) as u64;
        task::sleep(Duration::from_millis(ms)).await;
    }
}

async fn server(mut stream: TcpStream) {
    let mut data = [0u8; 16];
    loop {
        let n = match stream.read(&mut data).await {
            Ok(n) => n as i64,
            Err(e) => {
                println!("Error reading from stream: {e:?}");
                -1
            },
        };
        assert_ne!(n, 0);
    }
}

async fn listener(listener: TcpListener) {
    loop {
        let (stream, _) = listener.accept().await.expect("can't accept");
        task::spawn(server(stream));
    }
}

async fn main_async() {
    for i in 0..4 {
        let tcp_listener = TcpListener::bind(("127.0.0.1", 8080 + i)).await.expect("can't bind");
        task::spawn(listener(tcp_listener));

        task::yield_now().await;

        for j in 1..=25_000 {
            let stream = TcpStream::connect(("127.0.0.1", 8080 + i)).await
                .expect(&format!("can't connect to {}", 8080 + i));
            task::spawn(client(stream));

            if j % 1000 == 0 {
                println!("{i} — {j}");
            }
        }
    }

    task::sleep(Duration::from_secs(120)).await;
    println!("Done");
}
