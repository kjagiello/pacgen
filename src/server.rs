use chrono;
use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};

pub struct Config {
    pub addr: SocketAddr,
    pub pac: String,
}

struct Context {
    pub pac: String,
}

fn log_request<T, U>(remote_addr: &SocketAddr, req: &Request<T>, response: &Response<U>) {
    info!(
        "{} [{}] \"{} {} {:?}\" {}",
        remote_addr.ip(),
        chrono::offset::Local::now(),
        req.method(),
        req.uri(),
        req.version(),
        response.status().as_u16(),
    );
}

#[tokio::main]
pub async fn serve(config: Config) {
    let ctx = Arc::new(Context { pac: config.pac });

    let make_service = make_service_fn(move |conn: &AddrStream| {
        let ctx = ctx.clone();
        let remote_addr = conn.remote_addr();

        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let ctx = ctx.clone();

                async move {
                    let res = match *req.method() {
                        Method::GET => Response::builder()
                            .header("Content-Type", "application/x-ns-proxy-autoconfig")
                            .body(Body::from(ctx.pac.clone()))
                            .unwrap(),
                        _ => Response::builder()
                            .status(StatusCode::METHOD_NOT_ALLOWED)
                            .body(Body::empty())
                            .unwrap(),
                    };

                    log_request(&remote_addr, &req, &res);
                    Ok::<_, Error>(res)
                }
            }))
        }
    });

    info!("Starting PAC server at http://{}", config.addr);
    let server = Server::bind(&config.addr).serve(make_service);
    info!("Quit the server with CTRL-C");

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}
