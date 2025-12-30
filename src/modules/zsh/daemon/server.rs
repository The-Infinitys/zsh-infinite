use tokio::{
    io::AsyncReadExt,
    net::{UnixListener, UnixStream},
};

pub async fn server(socket_path: std::path::PathBuf) {
    // 古いソケットファイルを削除してBind
    let _ = tokio::fs::remove_file(&socket_path).await;
    let listener = UnixListener::bind(&socket_path).expect("Failed to bind socket");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Accept error: {}", e),
        }
    }
}

async fn handle_client(mut stream: UnixStream) -> Result<(), Box<dyn std::error::Error>> {
    use rkyv::from_bytes;
    use rkyv::rancor::Error;
    use rkyv::to_bytes;
    use tokio::io::AsyncWriteExt;
    use zsh_prompts::Commands;
    let mut len_buf = [0u8; 8];
    stream.read_exact(&mut len_buf).await?;
    let len = u64::from_le_bytes(len_buf) as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    let command = from_bytes::<Commands, Error>(&buf)?;
    let segments = command.exec();
    let bytes = to_bytes::<Error>(&segments)?;
    let res_len = (bytes.len() as u64).to_le_bytes();
    stream.write_all(&res_len).await?;
    stream.write_all(&bytes).await?;
    Ok(())
}
