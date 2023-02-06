mod handler;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::{TcpListener, TcpStream};

use crate::config::Config;

use handler::HttpHandler;

pub struct Server {
    config: Arc<Config>,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        Server { config }
    }

    pub async fn run(self) {
        let config = Arc::clone(&self.config);

        let addr = SocketAddr::new(config.host, config.port);
        let handler = HttpHandler::from(Arc::clone(&config));
        let server = Arc::new(self);

        // We start a loop to continuously accept incoming connections
        loop {
            let server = Arc::clone(&server);
            let handler = handler.clone();
            let listener = TcpListener::bind(addr).await.unwrap();
            let (stream, _) = listener.accept().await.unwrap();

            // Spawn a tokio task to serve multiple connections concurrently
            tokio::task::spawn(async move {
                let server = Arc::clone(&server);
                server.serve(stream, handler).await;
            });
        }
    }

    pub async fn serve(&self, stream: TcpStream, handler: HttpHandler) {
        let handler = handler.clone();

        let server = http1::Builder::new().serve_connection(
            stream,
            service_fn(|req| {
                let handler = handler.clone();
                async move {
                    info!("{} {}", req.method(), req.uri().path());
                    let req_time = Instant::now();

                    match st(handler.to_owned(), req).await {
                        Ok(res) => {
                            info!(
                                "Returned {} in {} ms",
                                res.status().as_str(),
                                req_time.elapsed().as_millis()
                            );
                            Ok(res)
                        }
                        Err(e) => Err(e),
                    }
                }
            }),
        );
        if let Err(e) = server.await {
            eprint!("Server error: {}", e);
        }
    }
}

pub async fn st(
    handler: HttpHandler,
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    handler.handle_request(req).await
}
