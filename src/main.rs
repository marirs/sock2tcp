use tokio::io;
use tokio::net::{TcpStream, UnixListener};

use std::fs;

static SOCKET_PATH: &str = "/tmp/sock2tcp-unix.sock";
static TCP_ADDR: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let listener = match UnixListener::bind(SOCKET_PATH) {
        Ok(m) => m,
        Err(_) => {
            fs::remove_file(SOCKET_PATH).unwrap();
            UnixListener::bind(SOCKET_PATH)?
        }
    };

    while let Ok((mut unixstream, _addr)) = listener.accept().await {
        tokio::spawn(async move {
            let mut tcpstream = TcpStream::connect(TCP_ADDR).await?;
            let (mut sck_rr, mut sck_wr) = unixstream.split();
            let (mut tcp_rr, mut tcp_wr) = tcpstream.split();

            let _ = tokio::join!(
                io::copy(&mut sck_rr, &mut tcp_wr),
                io::copy(&mut tcp_rr, &mut sck_wr)
            );

            Ok::<_, failure::Error>(())
        });
    }

    Ok(())
}