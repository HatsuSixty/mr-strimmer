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

build_image_rs() {
    cargo build --manifest-path=./image-rs/Cargo.toml

    image_rs_path=""
    if [ -f ./image-rs/target/debug/image-rs ]; then
        image_rs_path=./image-rs/target/debug/image-rs
    elif [ -f ./image-rs/target/release/image-rs ]; then
        image_rs_path=./image-rs/release/debug/image-rs
    else
        echo "ERROR: Could not find `image-rs` binary path" 1>&2
    fi

    mkdir -p ./bin

    cp $image_rs_path ./bin/
}

build_text_rs() {
    cargo build --manifest-path=./text-rs/Cargo.toml

    text_rs_path=""
    if [ -f ./text-rs/target/debug/text-rs ]; then
        text_rs_path=./text-rs/target/debug/text-rs
    elif [ -f ./text-rs/target/release/text-rs ]; then
        text_rs_path=./text-rs/release/debug/text-rs
    else
        echo "ERROR: Could not find `text-rs` binary path" 1>&2
    fi

    mkdir -p ./bin
    mkdir -p ./assets

    cp $text_rs_path ./bin/

    if [ -f ./text-rs/Cantarell.ttf ]; then
        cp ./text-rs/Cantarell.ttf ./assets/
    else
        echo "ERROR: Could not find file `Cantarell.ttf`" 1>&2
    fi
}

build_project() {
    cargo build
    if [ "$1" = "run" ]; then
        shift 1
        cargo run $@
    fi
}

if [ "$1" = "clean" ]; then
    find . -iname "*~" -exec rm {} +
    rm -rf bin/
elif [ "$1" = "clean_all" ]; then
    find . -iname "*~" -exec rm {} +
    rm -rf bin/
    git clean -fdx
else
    build_webcam_rs
    build_image_rs
    build_text_rs
    build_project $@
fi
