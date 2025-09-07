use std::collections::HashMap;

use rust_mini_redis::proc_cmd;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bind server to address
    let listen = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Listening on localhost:6379");

    while let Ok((mut socket, addr)) = listen.accept().await {
        println!("New client connected: {}", addr);

        tokio::spawn(async move {
            let mut buf = [0u8; 512];
            let mut storage = HashMap::<String, String>::new();

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Client {} disconnected", addr);
                        return;
                    }
                    Ok(byte_read) => {
                        // echo back client's message
                        let client_cmd = String::from_utf8_lossy(&buf[..byte_read]);
                        match proc_cmd(&client_cmd, &mut storage) {
                            Ok(res) => socket
                                .write_all(format!("{}\n", res).as_bytes())
                                .await
                                .expect("Failed to write res message"),
                            Err(e) => socket
                                .write_all(format!("{}\n", e).as_bytes())
                                .await
                                .expect("Failed to write err message"),
                        }
                        println!("{:?}", &storage);
                    }
                    Err(e) => {
                        eprintln!("Failed to read from {}: {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
    Ok(())
}
