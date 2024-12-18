#!/bin/bash

set -e

mkdir -p bin
mkdir -p build
pushd build
cmake .. -DCMAKE_INSTALL_PREFIX=.. -DCMAKE_EXPORT_COMPILE_COMMANDS=1
mv compile_commands.json ../
make
make install
popd
echo "Binary installed to $(realpath bin)"
