#!/bin/sh

wasm-pack build -t web --dev
python -m http.server --bind 127.0.0.1 8000
