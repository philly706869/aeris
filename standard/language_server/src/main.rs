use std::{fs, path::PathBuf};

use aeris_constants::*;
use aeris_standard_language_server::*;
use anyhow::Context;
use clap::Parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixListener,
    runtime::Builder as RuntimeBuilder,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to target project
    project_path: PathBuf,

    /// Number of worker threads
    #[arg(short, long)]
    worker_threads: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let project_path = args.project_path.canonicalize().context("")?;

    let config_path = project_path.join(CONFIG_FILE_NAME);
    let socket_path = project_path.join(DAEMON_SOCK_NAME);

    let config_content = fs::read_to_string(&config_path).context("")?;

    let runtime = {
        let mut builder = RuntimeBuilder::new_multi_thread();
        builder.enable_all();
        if let Some(worker_threads) = args.worker_threads {
            builder.worker_threads(worker_threads);
        }
        builder.build().context("")
    }?;

    runtime.block_on(async {
        let listener = UnixListener::bind(&socket_path).context("")?;

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

        Ok(())
    })
}
