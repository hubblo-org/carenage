FROM rust:1-alpine3.19 AS planner

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
RUN cargo build --manifest-path /carenage/carenage-cli/Cargo.toml
RUN cargo build --manifest-path /carenage/carenaged/Cargo.toml

FROM alpine:3.19 AS runtime

WORKDIR carenage

COPY --from=builder /carenage/target/release/carenage-cli /usr/local/bin
COPY --from=builder /carenage/target/release/carenaged /usr/local/bin

ENTRYPOINT ["/usr/local/bin/carenage-cli", "start"]
