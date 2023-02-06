use core::fmt;
use std::error::Error;

use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response, StatusCode};

use mime_guess;

use super::Handler;

pub struct FileResolveError(());

impl Error for FileResolveError {}

impl fmt::Debug for FileResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileResolveError").finish()
    }
}

impl fmt::Display for FileResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt("file resolve error", f)
    }
}

pub struct FileServer {
    root: PathBuf,
}

impl FileServer {
    pub fn new(root: PathBuf) -> Self {
        FileServer { root }
    }

    fn resolve_file(&self, path: &str) -> Result<PathBuf, FileResolveError> {
        let path = path.replace("//", "/");

        let full_path = if path.starts_with("/") {
            self.root.join(&path[1..])
        } else {
            self.root.join(path)
        };

        if full_path.is_file() {
            return Ok(full_path);
        }

        if full_path.is_dir() {
            return Ok(full_path.join("index.html"));
        }

        Err(FileResolveError(()))
    }

    pub async fn resolve(
        &self,
        path: &str,
    ) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error>> {
        let full_path = match self.resolve_file(path) {
            Ok(p) => p,
            Err(e) => return Err(Box::new(e)),
        };
        let fp = full_path.clone();

        let mut file = File::open(full_path).await?;

        let mime = mime_guess::from_path(fp).first_or_octet_stream();

        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;

        let resp = Response::builder()
            .header("Content-Type", mime.to_string())
            .body(Full::new(Bytes::from(contents)));

        Ok(resp.unwrap())
    }
}

pub struct FileServerHandler {
    file_server: Arc<FileServer>,
}

impl FileServerHandler {
    pub fn new(file_server: FileServer) -> Self {
        let file_server = Arc::new(file_server);
        FileServerHandler { file_server }
    }
}

#[async_trait]
impl Handler for FileServerHandler {
    async fn handle(&self, req: Request<hyper::body::Incoming>) -> Response<Full<Bytes>> {
        let path = req.uri().path();

        match self.file_server.resolve(path).await {
            Ok(res) => res,
            Err(_e) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("")))
                .unwrap(),
        }
    }
}
