extern crate hyper;
extern crate futures;

use hyper::StatusCode;
use hyper::header::{ContentLength, Location, Server};
use hyper::server::{Http, Request, Response, Service};

struct Redirect;

const LOCATION: &'static str = "https://edac.io";

impl Service for Redirect {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        // Returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the redrected path
        let location = LOCATION.to_string() + &(req.path())[..];
        futures::future::ok(
            Response::new()
                .with_status(StatusCode::Found)
                .with_header(Server::new("EDAC"))
                .with_header(Location::new(location))
                .with_header(ContentLength(0u64))
        )
    }
}

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Redirect)).unwrap();
    server.run().unwrap();
}
