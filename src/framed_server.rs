use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use std::env;
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8888".to_string());
    print!("Listening on: {}", addr);
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let mut framed_stream = Framed::new(stream, LengthDelimitedCodec::new());

        tokio::spawn(async move {
            while let Some(msg) = framed_stream.next().await {
                match msg {
                    Ok(msg) => {
                        let directive = String::from_utf8(msg.to_vec())
                            .expect("error when converting to string directive");
                        print!("{directive}");
                        let output = process(&directive).await;
                        print!("{output}");
                        _ = framed_stream.send(Bytes::from(output)).await;
                    }
                    Err(e) => {
                        print!("{e:?}");
                    }
                }
            }
        });
    }
}

async fn process(directive: &str) -> String {
    if directive == "gettime" {
        // 这里我们用了unwrap()是因为我们一般确信执行date命令不会失败
        // 更可靠的作法是对返回的Result作处理  解决Windows环境 date命令的问题
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "echo %date:~0,4%-%date:~5,2%-%date:~8,2%"])
                .output()
                .await.unwrap()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("date")
                .output()
                .await.unwrap()
        };
        String::from_utf8(output.stdout).unwrap()
    } else {
        // 如果是其它指令，我们目前返回 无效指令
        "invalid command".to_owned()
    }
}