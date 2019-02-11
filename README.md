# redirector
A simple asynchronous Rust/hyper based webserver to redirect http requests

## Environment variables:
* `LOCATION` Required value that gets placed in the http Location header. Example: `https://edac.io`
* `SERVER` Optional value that gets placed in the http Server header. Default value is `Hyper`
* `ADDRESS` Optional value that the service will listen to. Default value is `0.0.0.0:8080`
* `HEALTH` Required for health application to know what address to probe for health checks. Example: `http://edac.io/`
