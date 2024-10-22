#! /bin/sh

MANIFEST_PATH="/builds/hubblo/carenage/carenage/Cargo.toml"

apk update 
apk add curl gcc libressl-dev musl-dev pkgconf 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

cargo build --manifest-path $MANIFEST_PATH \
&& cargo build --manifest-path $MANIFEST_PATH -p carenaged \
&& cargo build --manifest-path $MANIFEST_PATH -p carenage-cli
