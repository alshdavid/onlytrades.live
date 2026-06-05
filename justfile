build:
  node .scripts/build.mts 
  cargo build

build_release:
  node .scripts/build.mts 
  cargo build --release

# Run "cargo install cargo-watch"
watch:
  cargo watch \
    -w "client" \
    -w "crates" \
    -w "examples_bots" \
    -w ".scripts" \
    -s "node .scripts/build.mts && cargo build && cargo run -p onlytrades"

# Run "cargo install cargo-xfmt"
fmt:
  npx prettier -w ./client/**/* 
  cargo xfmt

lint:
  cargo clippy
  npx tsc