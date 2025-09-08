use std::{collections::HashMap, sync::Arc};

use rust_mini_redis::utils::{aof_handler, cmd_handler};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::RwLock,
};

type SharedDB = Arc<RwLock<HashMap<String, String>>>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bind server to address
    let listen = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Listening on localhost:6379");

    let mut arc_storage: SharedDB = Arc::new(RwLock::new(HashMap::new()));
    aof_handler::startup_load(&mut arc_storage).await;

    // Handle each connection
    while let Ok((mut socket, addr)) = listen.accept().await {
        println!("New client connected: {}", addr);

        let mut storage_clone = Arc::clone(&arc_storage);

        // Handle client's communication
        tokio::spawn(async move {
            let mut buf = [0u8; 512];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Client {} disconnected", addr);
                        return;
                    }
                    Ok(byte_read) => {
                        let client_cmd = String::from_utf8_lossy(&buf[..byte_read]);
                        match cmd_handler::proc_cmd(&client_cmd, &mut storage_clone, false).await {
                            Ok(res) => socket
                                .write_all(format!("{}\n", res).as_bytes())
                                .await
                                .expect("Failed to write res message"),
                            Err(e) => socket
                                .write_all(format!("{}\n", e).as_bytes())
                                .await
                                .expect("Failed to write err message"),
                        }
                        println!("{:?}", &storage_clone.read().await);
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
