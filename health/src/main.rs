extern crate hyper;
extern crate futures;
#[macro_use] extern crate lazy_static;

use futures::Future;
use hyper::{rt, Client, StatusCode, Uri};
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

fn fetch_url(url: hyper::Uri) -> impl Future<Item=(), Error=()> {
    let client = Client::new();
    client
        .get(url)
        .map(|res| {
            if res.status() != StatusCode::FOUND {
                println!("Invalid redirect status returned: {}", res.status());
                std::process::exit(1);
            }
            if let Some(location) = res.headers().get("location") {
                match location.to_str() {
                    Ok(l) => if l != &*LOCATION {
                        println!("Invalid redirect location returned {}", l);
                        std::process::exit(1);
                    },
                    Err(e) => {
                        println!("Error reading location header {}", e);
                        std::process::exit(1);
                    },
                }
            } else {
                println!("No location header found");
                std::process::exit(1);
            }
        })
        .map_err(|err| {
            println!("Error: {}", err);
        })
}

fn main() {
    let url: Uri = match std::env::var("HEALTH") {
        Ok(a) => a.parse().unwrap(),
        Err(_)  => "0.0.0.0:8080".parse().unwrap(),
    };
    rt::run(fetch_url(url));
}
