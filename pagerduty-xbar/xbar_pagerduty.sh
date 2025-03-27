#!/bin/bash
set -a
source $PWD/.pg_env
set +a
cargo run | tee response.json
