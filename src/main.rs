#[macro_use]
extern crate structopt;
extern crate hyper;

use hyper::rt::Future;
use hyper::{Body, Request, Response, Server};
use structopt::StructOpt;

const PHRASE: &str = "Hello, World!";

#[derive(StructOpt, Debug)]
#[structopt(about = "Web news server")]
struct Config {
    #[structopt(long = "listen", help = "HTTP listen address", default_value = "127.0.0.1:8080")]
    listen: std::net::SocketAddr,
}

struct Handler {}

impl Handler {
    fn new() -> Handler {
        Handler {}
    }

    fn serve(&self, _req: Request<Body>) -> std::result::Result<Response<Body>, &'static str> {
        std::result::Result::Ok(Response::new(Body::from(PHRASE)))
    }
}

fn main() {
    let cfg = Config::from_args();
    println!("Listening to {:?}", cfg.listen);

    let server_builder = match Server::try_bind(&cfg.listen) {
        std::result::Result::Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
        std::result::Result::Ok(res) => res,
    };
    let server = server_builder.serve(move || {
        let handler: Handler = Handler::new();
        hyper::service::service_fn(move |req| handler.serve(req))
    });

    // Run this server for... forever!
    hyper::rt::run(server.map_err(|e| eprintln!("Server error: {}", e)));
}
