use std::path::PathBuf;

use tokio::{
    net::{UnixListener, UnixStream},
    runtime::Runtime,
};

pub fn main() {
    let root =
        AERISDaemon::find_project_root_from_current_dir().expect("Failed to find project root");
    let daemon = AERISDaemon::new(root);
    daemon.start().unwrap();
}

pub struct AERISDaemon {
    root_path: PathBuf,
}

impl AERISDaemon {
    const UVCS_PATH: &str = ".uvcs/aeris/daemon";
    const UDS_NAME: &str = ".sock";

    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
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

    pub fn start(&self) -> std::io::Result<()> {
        let runtime = Self::create_runtime()?;
        let listener = self.create_listener()?;
        runtime.block_on(Self::accept_loop(&listener));
        Ok(())
    }

    fn create_runtime() -> std::io::Result<Runtime> {
        tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .build()
    }

    fn create_listener(&self) -> std::io::Result<UnixListener> {
        let mut path = self.root_path.clone();
        path.push(Self::UVCS_PATH);
        path.push(Self::UDS_NAME);
        UnixListener::bind(&path)
    }

    async fn accept_loop(listener: &UnixListener) {
        loop {
            match listener.accept().await {
                Ok((socket, _)) => tokio::spawn(Self::handler(socket)),
                Err(_) => continue,
            };
        }
    }

    async fn handler(mut _socket: UnixStream) {
        todo!()
    }
}
