use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response, Server};

pub struct Config {
    pub addr: SocketAddr,
    pub pac: String,
}

struct Context {
    pub pac: String,
}

#[tokio::main]
pub async fn serve(config: Config) {
    let ctx = Arc::new(Context { pac: config.pac });

    let make_service = make_service_fn(move |_| {
        let ctx = ctx.clone();

        async move {
            Ok::<_, Error>(service_fn(move |_req| {
                let ctx = ctx.clone();

                async move {
                    let response = Response::builder()
                        .header("Content-Type", "application/x-ns-proxy-autoconfig")
                        .body(Body::from(ctx.pac.clone()))
                        .unwrap();

                    Ok::<_, Error>(response)
                }
            }))
        }
    });

    info!("Starting PAC server at http://{}", config.addr);
    let server = Server::bind(&config.addr).serve(make_service);
    info!("Quit the server with CTRL-C");

    if let Err(e) = server.await {
        error!(target: "server", "server error: {}", e);
    }
}
