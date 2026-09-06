use rust_pty::{NativePtySystem, PtyConfig, PtySystem};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = PtyConfig::default();
    let (mut master, mut child) = NativePtySystem::spawn_shell(&config).await?;

    // Write a command
    master.write_all(b"echo hello\n").await?;

    // Read output
    let mut buf = [0u8; 1024];
    let n = master.read(&mut buf).await?;
    println!("{}", String::from_utf8_lossy(&buf[..n]));

    // Clean up
    child.kill()?;
    Ok(())
}
