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

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Client {} disconnected", addr);
                        return;
                    }
                    Ok(byte_read) => {
                        if let Err(e) = socket.write_all(&buf[..byte_read]).await {
                            eprintln!("Failed to write to {}: {}", addr, e);
                            return;
                        }
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
