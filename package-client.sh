#!/bin/bash
set -e
cd client
if [ "$CLIENT_WS_URL" == "" ]; then
  export CLIENT_WS_URL="wss://bellclone-server.maowtm.org"
fi
rm -rf pkg target/wasm32-unknown-unknown/release
./build.sh
tar -c pkg index.html bootstrap.js | gzip > ../client.tar.gz
cd ..
