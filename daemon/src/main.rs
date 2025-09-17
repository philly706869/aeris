use std::fs;

use aeris_constants::*;
use aeris_standard_daemon::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixListener,
    runtime::Builder as RuntimeBuilder,
};

fn main() {
    let project_path = std::env::current_dir().unwrap();
    let config_path = project_path.join(CONFIG_FILE_NAME);
    let socket_path = project_path.join(DAEMON_SOCK_NAME);

    if socket_path.exists() {
        panic!("Socket already exists: {:?}", &socket_path)
    }

    let config_content =
        fs::read_to_string(&config_path).expect(&format!("Failed to read {:?}.", &config_path));

    let runtime = RuntimeBuilder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create runtime.");

    runtime.block_on(async {
        let listener = UnixListener::bind(&socket_path).unwrap();

        loop {
            let (mut socket, _) = match listener.accept().await {
                Ok(v) => v,
                Err(_) => break,
            };
            tokio::spawn(async move {
                let mut buf = vec![0; 1024];
                let len = match socket.read(&mut buf).await {
                    Ok(v) => v,
                    Err(_) => return,
                };
                println!("Received: {}", String::from_utf8_lossy(&buf[..len]));

                let _ = socket.write_all(b"Hello, World!").await;
            });
        }
    });
}
