# GCS Proxy

<center><img src=./img/logo.png></center>

<center><a href="https://github.com/guaychou/gcs-proxy/workflows/github_action.yaml">
<img src="https://github.com/guaychou/gcs-proxy/workflows/ci/badge.svg">
</a>
</center>

---
GCS proxy is tools to download file in google cloud storage without knowing the google cloud storage url, and abstract it with http basic auth feature in rust

## Routes

- /_/download/{file_name}

## Example Yaml Config
```yaml
log_level: info # debug / info / warn / error
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
- CONFIG_PATH --> Path to config file

## How to run example

```bash
$ docker run -p8080:8080 -d -v "./config.yaml:/app/config.yaml" ghcr.io/guaychou/gcs-proxy:v1.0.0
```

## Credit
- [Zero To Production](https://www.zero2prod.com/) for Architect the application code
- [Actix Web](https://actix.rs/) for the stable, blazingly fast, and battle tested web server written in rust with actor model

## Maintainer

- Kevin Jonathan Harnanta | <kevin.harnanta@gmail.com>
