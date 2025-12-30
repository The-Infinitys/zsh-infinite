use super::paths::get_daemon_paths;
use rkyv::rancor::Error;
use rkyv::util::AlignedVec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use zsh_prompts::{Commands, PromptSegment}; // 前に作成したパス取得関数

pub async fn get(command: &Commands) -> Vec<PromptSegment> {
    let paths = get_daemon_paths();

    // 1. デーモン（サーバー）に接続
    let mut stream = match UnixStream::connect(&paths.socket).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Daemon not found: {}. Is it running?", e);
            return Vec::new();
        }
    };

    // 2. Commands を rkyv でシリアライズ
    let request_bytes = match rkyv::to_bytes::<Error>(command) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Serialization error: {:?}", e);
            return Vec::new();
        }
    };

    // 3. サイズ -> 本体の順で送信
    let len_prefix = (request_bytes.len() as u64).to_le_bytes();
    if stream.write_all(&len_prefix).await.is_err()
        || stream.write_all(&request_bytes).await.is_err()
    {
        return Vec::new();
    }

    // 4. サーバーからのレスポンスサイズ (8 bytes) を受信
    let mut res_len_buf = [0u8; 8];
    if stream.read_exact(&mut res_len_buf).await.is_err() {
        return Vec::new();
    }
    let res_len = u64::from_le_bytes(res_len_buf) as usize;

    // 5. AlignedVec で本体 (Vec<PromptSegment>) を受信
    let mut res_buf = AlignedVec::<16>::with_capacity(res_len);
    unsafe {
        res_buf.set_len(res_len);
    }
    if stream.read_exact(&mut res_buf).await.is_err() {
        return Vec::new();
    }

    // 6. デシリアライズして結果を返す
    match rkyv::from_bytes::<Vec<PromptSegment>, Error>(&res_buf) {
        Ok(segments) => segments,
        Err(e) => {
            eprintln!("Deserialization error: {:?}", e);
            Vec::new()
        }
    }
}
