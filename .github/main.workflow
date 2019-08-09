workflow "Run test on push" {
  resolves = ["Rust Action"]
  on = "push"
}

action "Rust Action" {
  uses = "icepuma/rust-action@1.0.9"
  args = "cargo fmt -- --check && cargo test"
}
