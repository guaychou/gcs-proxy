# GCS Proxy


## What is this
```
GCS proxy is tools to download file in gcs without knowing the gcs url, and abstract it with http basic auth feature in rust
```

## Project Status --> Beta

## Example Yaml Config
```yaml
server:
  http:
    port: 8080
    shutdown_timeout: 30
  gcs:
    bucket: kepintez
    folder: "folder_to_jail"
    # service_account_b64: "exampleofbase64sa"
  auth: 
    username: pogpog
    password: pogpog
```

## Environment Variable
- GCS_SA_B64 --> GCP Service Account Base64 encoded from json

## Credit
- [Zero To Prodcution](https://www.zero2prod.com/) for Architect the application code
- [Actix Web](https://actix.rs/) for the stable, blazingly fast, and battle tested web server written in rust with actor model

## Maintainer

- Kevin Jonathan Harnanta | <kevin.harnanta@gmail.com>