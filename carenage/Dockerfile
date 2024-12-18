FROM rust:1-alpine3.19 AS planner

LABEL org.opencontainers.image.authors="contact@hubblo.org"
LABEL org.opencontainers.image.description="Docker image for Carenage, a CI tool for environmental evaluation of software projects"
LABEL org.opencontainers.image.licenses=Apache-2.0
LABEL org.opencontainers.image.title="carenage-dashboard"

WORKDIR carenage
RUN apk add gcc libressl-dev musl-dev
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1-alpine3.19 AS cacher

WORKDIR carenage
RUN apk add gcc libressl-dev musl-dev
RUN cargo install cargo-chef
COPY --from=planner /carenage/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1-alpine3.19 AS builder

WORKDIR carenage
RUN apk add gcc libressl-dev musl-dev
COPY . .
COPY --from=cacher /carenage/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --manifest-path /carenage/carenage-cli/Cargo.toml --release --target x86_64-unknown-linux-musl
RUN cargo build --manifest-path /carenage/carenaged/Cargo.toml --release --target x86_64-unknown-linux-musl
RUN cargo build --manifest-path /carenage/api/Cargo.toml --release --target x86_64-unknown-linux-musl

FROM alpine:3.19 AS runtime

WORKDIR carenage

COPY --from=builder /carenage/target/x86_64-unknown-linux-musl/release/carenage-cli /usr/local/bin
COPY --from=builder /carenage/target/x86_64-unknown-linux-musl/release/carenaged /usr/local/bin
COPY --from=builder /carenage/target/x86_64-unknown-linux-musl/release/api /usr/local/bin

CMD ["/usr/local/bin/api"]
