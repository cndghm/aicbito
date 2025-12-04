use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("irc.libera.chat:6667").await?;
    println!("Connected.");
    Ok(())
}
