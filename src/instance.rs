use crate::error::{FairError, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};

pub const SOCKET_PATH: &str = "/run/fair.sock";
const SHOW_MESSAGE: &[u8] = b"show";

pub enum Instance {
    Primary(UnixListener),
    Secondary,
}

pub async fn acquire() -> Result<Instance> {
    if notify_existing().await {
        return Ok(Instance::Secondary);
    }

    let _ = std::fs::remove_file(SOCKET_PATH);
    let listener = UnixListener::bind(SOCKET_PATH).map_err(FairError::from)?;
    Ok(Instance::Primary(listener))
}

async fn notify_existing() -> bool {
    let Ok(mut stream) = UnixStream::connect(SOCKET_PATH).await else {
        return false;
    };
    stream.write_all(SHOW_MESSAGE).await.is_ok()
}

pub async fn serve(listener: UnixListener, on_show: impl Fn() + Send + 'static) {
    loop {
        let Ok((mut stream, _)) = listener.accept().await else {
            continue;
        };
        let mut buf = [0u8; 8];
        if stream.read(&mut buf).await.is_ok() {
            on_show();
        }
    }
}