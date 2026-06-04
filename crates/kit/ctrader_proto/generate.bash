#!/usr/bin/env bash

# The generated files are committed because
# the upstream code doesn't change much and
# it's annoying to generate these types

# 1) Install protoc
# 2) Run "cargo install protoc-gen-prost"

rm -rf ./lib.rs

protoc --plugin=protoc-gen-prost=$(which protoc-gen-prost) \
  --proto_path=openapi-proto-messages \
  --prost_out=. \
  openapi-proto-messages/*.proto

mv ./_ ./lib.rs
cargo xfmt