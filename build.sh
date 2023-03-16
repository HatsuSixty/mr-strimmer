#!/bin/sh

set -xe

build_webcam_rs() {
    cargo build --manifest-path=./webcam-rs/Cargo.toml

    webcam_rs_path=""
    if [ -f ./webcam-rs/target/debug/webcam-rs ]; then
        webcam_rs_path=./webcam-rs/target/debug/webcam-rs
    elif [ -f ./webcam-rs/target/release/webcam-rs ]; then
        webcam_rs_path=./webcam-rs/release/debug/webcam-rs
    else
        echo "ERROR: Could not find `webcam-rs` binary path" 1>&2
    fi

    mkdir -p ./bin

    cp $webcam_rs_path ./bin/
}

build_project() {
    cargo build
    if [ "$1" = "run" ]; then
        shift 1
        cargo run $@
    fi
}

build_webcam_rs
build_project $@
