#!/usr/bin/env bash

cargo +nightly web start -p kreida-wasm-demo --release --auto-reload --target=wasm32-unknown-unknown
