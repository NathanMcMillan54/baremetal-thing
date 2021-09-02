#!/bin/bash

cargo build --target thumbv7em-none-eabihf --release
mv target/thumbv7em-none-eabihf/release/thing thing.img
