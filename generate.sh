#!/usr/bin/env zsh

cargo build --release
PATH=$PATH:$(pwd)/target/release vhs --publish twacqwq.tape