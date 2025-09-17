use std::env;

use aeris_constants::*;
use aeris_standard_cli::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
    runtime::Builder as RuntimeBuilder,
};

fn main() {
    let current_path = env::current_dir().expect("Cannot get current directory.");
    let (root_path, config_path) = find_nearest_path(&current_path, CONFIG_FILE_NAME)
        .expect(&format!("Cannot find {}.", CONFIG_FILE_NAME));
    let daemon_socket_path = root_path.join(DAEMON_SOCK_NAME);

    if !daemon_socket_path.exists() {
        panic!("Cannot find daemon socket. ({:#?})", daemon_socket_path)
    }

    dbg!(current_path);
    dbg!(root_path);
    dbg!(config_path);
    dbg!(&daemon_socket_path);

    let runtime = RuntimeBuilder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let _: std::io::Result<()> = runtime.block_on(async {
        let mut stream = UnixStream::connect(&daemon_socket_path).await?;
        stream.write_all(b"Hello, World!").await?;
        let mut buf = vec![0; 1024];
        let len = stream.read(&mut buf).await?;
        println!("Received: {}", String::from_utf8_lossy(&buf[..len]));
        Ok(())
    });
}
