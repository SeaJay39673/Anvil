use rust_pty::{NativePtySystem, PtyConfig, PtySystem};
use tokio::io::{
    AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadHalf, WriteHalf,
};

mod terminal_core;

async fn read_output<R: AsyncRead>(mut reader: ReadHalf<R>) -> anyhow::Result<()> {
    let mut buf = [0u8; 4096];
    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        print!("{}", String::from_utf8_lossy(&buf[..n]));
    }

    Ok(())
}

async fn write_output<W: AsyncWrite>(mut writer: WriteHalf<W>) -> anyhow::Result<()> {
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut input = String::new();

    loop {
        input.clear();

        stdin.read_line(&mut input).await?;

        if input.trim() == "exit" {
            break;
        }

        writer.write_all(input.as_bytes()).await?;
    }

    anyhow::Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = PtyConfig::default();
    let (master, mut child) = NativePtySystem::spawn_shell(&config).await?;

    let (reader, writer) = tokio::io::split(master);

    tokio::spawn(read_output(reader));
    write_output(writer).await?;

    child.kill()?;
    Ok(())
}
