#!/bin/bash

env RUST_BACKTRACE=1 RUSTC_BOOTSTRAP=1 SHARING_DATA=$(pwd)/SharingData.txt cargo run --features emulate -- --no-capture