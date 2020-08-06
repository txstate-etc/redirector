#[macro_use] extern crate lazy_static;
use std::fmt;
use std::env;
use hyper::{Client, StatusCode, Uri};
use std::error::Error;

// LOCATION is what should be found in the location header returned back from redirector;
// example: LOCATION=https://edac.io
lazy_static! {
    static ref LOCATION: String = {
        match env::var("LOCATION") {
            Ok(location) => location + "/health",
            Err(_) => panic!("No redirect location specified"),
        }
    };
}

// HEALTH environment variable holds url to test redirector
lazy_static! {
    static ref HEALTH: String = {
        match env::var("HEALTH") {
            Ok(health) => health + "/health",
            Err(_) => "http://localhost:3000/health".to_string(),
        }
    };
}

#[derive(Debug)]
enum HealthCheckError {
    NonRedirect(StatusCode),
    //println!("Non redirect status returned: {}", res.status());
    InvalidLocation(String),
    //println!("Invalid redirect location returned {}", l);
    UnreadableLocation(hyper::header::ToStrError),
    //println!("Error reading location header {}", e);
    NoLocationFound,
    //println!("No location header found");
}

impl fmt::Display for HealthCheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthCheckError::NonRedirect(status_code) => write!(f, "Non redirect status returned: {}", status_code),
            HealthCheckError::InvalidLocation(location) => write!(f, "Invalid redirect location returned {}", location),
            HealthCheckError::UnreadableLocation(error) => write!(f, "Error reading location header {}", error),
            HealthCheckError::NoLocationFound => write!(f, "No location header found"),
        }
    }
}

impl Error for HealthCheckError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HealthCheckError::NonRedirect(_status_code) => None,
            HealthCheckError::InvalidLocation(_location) => None,
            HealthCheckError::UnreadableLocation(error) => Some(error),
            HealthCheckError::NoLocationFound => None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url: Uri = (&*HEALTH).parse().unwrap();
    let client = Client::new();
    let res = client.get(url).await?;
    if res.status() != StatusCode::FOUND {
        Err(HealthCheckError::NonRedirect(res.status()))?
    } else if let Some(location) = res.headers().get("location") {
        match location.to_str() {
            Ok(l) => if l != &*LOCATION {
                Err(HealthCheckError::InvalidLocation(l.to_string()))?
            } else {
                Ok(())
            },
            Err(e) => {
                Err(HealthCheckError::UnreadableLocation(e))?
            },
        }
    } else {
        Err(HealthCheckError::NoLocationFound)?
    }
}
