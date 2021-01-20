#!/bin/sh

wasm-pack build -t web --dev && \
python -m http.server --bind 172.17.0.2 8000
