# redirector
A simple redirect server

## Environment variables:
* `LOCATION` Required value that gets placed in the http Location header. Example: `https://edac.io`
* `SERVER` Optional value that gets placed in the http Server header. Default value is `Hyper`
* `ADDRESS` Optiona value that the service will listen to. Default value is `0.0.0.0:8080`
