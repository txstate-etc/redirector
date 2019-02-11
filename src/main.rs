extern crate hyper;
extern crate futures;
#[macro_use] extern crate lazy_static;

use futures::{future, Future};
use hyper::{http::response::Builder, service::service_fn, Body, Request, Response, Server, StatusCode};
use std::env;

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

fn redirect(req: Request<Body>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    // Build a Response with the redrected path
    let location = (&*LOCATION).to_string() + &(req.uri().path())[..];
    Box::new(future::ok(Builder::new()
        .status(StatusCode::FOUND)
        .header("server", (&*SERVER).to_string())
        .header("location", location)
        .body(Body::empty())
        .unwrap()))
}

fn main() {
    let address = match std::env::var("ADDRESS") {
        Ok(a) => a.parse().unwrap(),
        Err(_)  => "0.0.0.0:8080".parse().unwrap(),
    };
    let server = Server::bind(&address)
        .serve(|| service_fn(redirect))
        .map_err(|e| eprintln!("ERROR: {}", e));
    println!("Listening to {}", address);
    hyper::rt::run(server);
}
