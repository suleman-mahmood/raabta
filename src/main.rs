use std::net::TcpListener;

use forge::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let _ = run(listener)?;
    Ok(())
}
