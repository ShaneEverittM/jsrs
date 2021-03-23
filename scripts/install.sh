#!/bin/bash

cargo build --release
cp "$HOME"/dev/rust/javascript-rs/target/release/javascript-rs "$HOME"/.local/bin/js
