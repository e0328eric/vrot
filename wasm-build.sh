#! /bin/sh

# Copyright (c) 2022 Sungbae Jeong
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

wasm-pack build --target web -d vrot-web/pkg
tsc -p tsconfig.json && tsc-alias -p tsconfig.json
rm vrot-web/pkg/.gitignore