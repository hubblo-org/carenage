#! /bin/bash

set -o nounset
set -o noclobber
export LC_ALL=C
export PATH="/bin:/sbin:/usr/bin:/usr/sbin:$PATH"
PS4=' ${BASH_SOURCE##*/}:$LINENO ${FUNCNAME:-main}) '

apt update && apt install -y gcc pkg-config libssl-dev
cargo build && cargo build -p carenaged && cargo build -p carenage-cli
