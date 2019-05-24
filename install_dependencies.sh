#!/bin/bash

set -e

brew install zeromq
cargo install diesel_cli --no-default-features --features="postgres"