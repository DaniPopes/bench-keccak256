#!/bin/sh

set -e
cd "$(mktemp)"
git clone git@github.com:DaniPopes/keccak-asm
git clone git@github.com:DaniPopes/bench-keccak256
cd bench-keccak256
