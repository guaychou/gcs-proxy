FROM rust:1.67-slim-bullseye as build-env
WORKDIR /app
ADD . /app
RUN apt update -y && apt install -y make libssl-dev pkg-config
RUN rustup component add rustfmt clippy
RUN cargo fmt --all -- --check  && cargo clippy
RUN RUSTFLAGS="-C link-arg=-s --cfg unsound_local_offset" cargo build --locked --no-default-features --release

FROM gcr.io/distroless/cc-debian11:nonroot-amd64
LABEL org.opencontainers.image.description "Google Cloud Storage Proxy for Download and provide limited access combined with http basic authentication, written in blazingly fast rust actix web framework"
ENV TZ="Asia/Jakarta"
WORKDIR /app
COPY --from=build-env /app/target/release/gcs-proxy-apps /app
CMD ["./gcs-proxy-apps"]