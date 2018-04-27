extern crate hyper;
extern crate futures;
extern crate tokio_core;
#[macro_use] extern crate lazy_static;

use futures::{Future, Stream};
use hyper::StatusCode;
use hyper::header::{ContentLength, Location, Server};
use hyper::server::{Http, Request, Response, Service};
use tokio_core::reactor::Core;
use std::env;

struct Redirect<'a> {
    location: &'a str,
    server: &'a str,
}

impl<'a> Service for Redirect<'a> {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        // Build a Response with the redrected path, and return
        // an 'ok' Future, which means it's immediatly ready.
        let location = self.location.to_string() + &(req.path())[..];
        futures::future::ok(
            Response::new()
                .with_status(StatusCode::Found)
                .with_header(Server::new(self.server.to_string()))
                .with_header(Location::new(location))
                .with_header(ContentLength(0u64))
        )
    }
}

//const LOCATION: &'static str = "https://edac.io";
// example: LOCATION=https://edac.io
lazy_static! {
    static ref LOCATION: String = {
        match env::var("LOCATION") {
            Ok(location) => location,
            Err(_) => panic!("No redirect location specified"),
        }
    };
}

lazy_static! {
    static ref SERVER: String = {
        match env::var("SERVER") {
            Ok(server) => server,
            Err(_) => "Hyper".to_string(),
        }
    };
}

fn main() {
    let address = match std::env::var("ADDRESS") {
        Ok(a) => a.parse().unwrap(),
        Err(_)  => "0.0.0.0:8080".parse().unwrap(),
    };
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let srv = Http::new().sleep_on_errors(true).serve_addr_handle(&address, &handle, || Ok(Redirect{ location: &LOCATION, server: &SERVER })).unwrap();
    println!("Listening to {}", address);
    let handle1 = handle.clone();
    handle.spawn(srv.for_each(move |conn| {
        handle1.spawn(conn.map(|_| ()).map_err(|err| println!("server error: {:?}", err)));
        Ok(())
    }).map_err(|_| ()));
    core.run(futures::future::empty::<(), ()>()).unwrap();
}
