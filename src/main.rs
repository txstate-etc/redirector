#[macro_use] extern crate lazy_static;
use std::env;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

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

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn redirect(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Build a Response with the redrected path
    let location = (&*LOCATION).to_string() + &(req.uri().path())[..];
    let res = Response::builder()
        .status(StatusCode::FOUND)
        .header("Server", (&*SERVER).to_string())
        .header("Location", location)
        .body(Body::empty());
    match res {
        Ok(res) => Ok(res),
        Err(error) => {
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Internal Server Error: {}", error))).unwrap())
        },
    }

}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(redirect))
    });
    let server = Server::bind(&addr).serve(svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());
    println!("Listening to {}", addr);
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
