use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listen = TcpListener::bind("127.0.0.1:6379")?;
    for stream in listen.incoming() {
        let stream = stream?;
        println!("{stream:?} connected");
    }
    Ok(())
}
