use std::path::PathBuf;

use aeris_std::StdContext;
use tokio::{
    io::AsyncReadExt,
    net::{UnixListener, UnixStream},
};

pub fn main() {
    let root =
        AERISDaemon::find_project_root_from_current_dir().expect("Failed to find project root");
    let daemon = AERISDaemon::new(root);
    daemon.listen().unwrap();
}

pub struct AERISDaemon {
    ctx: StdContext,
    root_path: PathBuf,
}

impl AERISDaemon {
    const UVCS_PATH: &str = ".uvcs/aeris/daemon";
    const UDS_NAME: &str = ".sock";

    pub fn new(root_path: PathBuf) -> Self {
        let ctx = StdContext::new();
        Self { ctx, root_path }
    }

    pub fn find_project_root_from_current_dir() -> std::io::Result<PathBuf> {
        let mut path = std::env::current_dir()?;
        loop {
            path.push("workspace.aeris");
            let exists = path.exists();
            path.pop();
            if exists {
                return Ok(path);
            }
            if !path.pop() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Project root not found",
                ));
            }
        }
    }

    pub fn listen(&self) -> std::io::Result<()> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .build()?;
        let mut path = self.root_path.clone();
        path.push(Self::UVCS_PATH);
        path.push(Self::UDS_NAME);
        let listener = UnixListener::bind(&path)?;
        runtime.block_on(Self::accept_loop(&listener))
    }

    async fn accept_loop(listener: &UnixListener) -> std::io::Result<()> {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(Self::handler(stream));
                }
                Err(e) => return Err(e),
            }
        }
    }

    async fn handler(mut stream: UnixStream) {
        let mut buf = [0u8; 1024];
        match stream.read(&mut buf).await {
            Ok(0) => {}
            Ok(len) => {}
            Err(err) => {}
        }
    }
}
