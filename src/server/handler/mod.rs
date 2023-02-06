mod file_server;

use async_trait::async_trait;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use std::convert::Infallible;
use std::sync::Arc;

use crate::config::Config;
use file_server::{FileServer, FileServerHandler};

#[async_trait]
pub trait Handler {
    async fn handle(&self, req: Request<hyper::body::Incoming>) -> Response<Full<Bytes>>;
}

#[derive(Clone)]
pub struct HttpHandler {
    handler: Arc<dyn Handler + Send + Sync>,
}

impl HttpHandler {
    pub async fn handle_request(
        self,
        req: Request<hyper::body::Incoming>,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        Ok(self.handler.handle(req).await)
    }
}

impl From<Arc<Config>> for HttpHandler {
    fn from(config: Arc<Config>) -> Self {
        let file_server = FileServer::new(config.public.clone());
        let handler = Arc::new(FileServerHandler::new(file_server));

        HttpHandler { handler }
    }
}
