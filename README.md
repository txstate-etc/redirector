# redirector
A simple asynchronous Rust/hyper based webserver to redirect http requests

## Environment variables:
* `LOCATION` Required value that gets placed in the http Location header. Example: `https://edac.io`
* `SERVER` Optional value that gets placed in the http Server header. Default value is `Hyper`
* `ADDRESS` Optional value that the service will listen to. Default value is `0.0.0.0:3001`
* `HEALTH` Optional URL for health application to know what address to probe for health checks. Example: `http://edac.io`, Default: `http://localhost:8080` Note that health check will append a `/health` path to the HEALTH URL in order to also check that the redirector includes this path in the location header.
