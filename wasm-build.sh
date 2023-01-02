#! /bin/sh

# Copyright (c) 2022 Sungbae Jeong
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

wasm-pack build --target web -d vrot-web/pkg
cp src/index.js vrot-web
rm vrot-web/pkg/.gitignore
