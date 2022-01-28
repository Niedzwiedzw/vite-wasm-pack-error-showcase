#!/usr/bin/env sh
set -e
cd some-rust-wasm-package
rm ./pkg/*
wasm-pack build --debug --target web
wasm-pack pack
cd ..
npm remove some-rust-wasm-package
npm install
npm install ./some-rust-wasm-package/pkg/*.tgz
npm run dev