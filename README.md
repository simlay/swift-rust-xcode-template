# Swift Rust Xcode Template

[![Build Status](https://github.com/simlay/swift-rust-xcode-template/workflows/Template/badge.svg)](https://github.com/simlay/swift-rust-xcode-template/actions)

This is a template for quicking building an iOS app in rust and swift. It uses
cbindgen to build `bindings.h` for convenient rust-swift interop.

To use this with (cargo-generate)[https://github.com/ashleygwilliams/cargo-generate] do:
```
cargo generate --git https://github.com/simlay/swift-rust-xcode-template.git --name myproject
cd myproject
./rename.sh
```

The `./rename.sh` step renames the xcode stuff to match your project name.
