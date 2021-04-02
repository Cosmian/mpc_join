#!/bin/bash

env RUST_BACKTRACE=1 RUSTC_BOOTSTRAP=1 SHARING_DATA=$(pwd)/SharingData.txt RUST_TEST_THREADS=1 cargo test --features emulate -- --nocapture