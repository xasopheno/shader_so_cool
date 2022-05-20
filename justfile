test:
  cargo nextest run

dev:
  cargo watch --exec "run --release"

watch:
  cargo watch

run: 
  cargo run --release
