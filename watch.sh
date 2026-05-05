#!/bin/sh

if command -v "$1" >/dev/null 2>&1; then
    exec "$@"
fi

if ! command -v cargo-watch >/dev/null; then
    cargo install cargo-watch
fi

sleep 5

exec cargo watch "$@"
