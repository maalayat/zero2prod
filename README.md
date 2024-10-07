## Faster Linking
```toml
# .cargo/config.toml

# On Windows
# ```
# cargo install -f cargo-binutils
# rustup component add llvm-tools-preview
# ```
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# On Linux:
# - Ubuntu, `sudo apt-get install lld clang`
# - Arch, `sudo pacman -S lld clang`
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

# On MacOS, `brew install llvm` and follow steps in `brew info llvm`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld64.lld"]
```
## cargo check
```
cargo check
```

## cargo run
```
cargo run
```

## cargo-watch
```
cargo install cargo-watch
```
```
cargo watch -x check
```

```
cargo watch -x check -x test -x run
```

## Code Coverage
```
# sudo apt-get install libssl-dev
# sudo apt-get install pkg-config

cargo install cargo-tarpaulin
```
```
cargo tarpaulin --ignore-tests
```

## Linting
```
rustup component add clippy
```
```
cargo clippy 
```
```
cargo clippy -- -D warnings
```

## Formatting
```
# format your whole project
cargo fmt
```
```
# CI 
cargo fmt -- --check
```

## Security Vulnerabilities
```
cargo install cargo-audit
```
```
cargo audit
```

## Choosing A Web Framework
- [actix-web’s website](https://actix.rs/)
- [actix-web ’s documentation](https://docs.rs/actix-web/4.0.1/actix_web/index.html)
- [actix-web’s examples collection](https://github.com/actix/examples)