#!/bin/bash

brew install zeromq || true
cargo install diesel_cli --no-default-features --features="postgres" --quiet || true

