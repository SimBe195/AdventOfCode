#!/bin/bash

set -e

mkdir -p bin
mkdir -p build
pushd build
cmake .. -DCMAKE_INSTALL_PREFIX=..
make
make install
popd
echo "Binary installed to $(realpath bin)"
