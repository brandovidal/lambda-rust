# Lambda Rust

### 📌 Pre requisites

[Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) in your system


[Vs Code Extension](https://marketplace.visualstudio.com/items?itemName=1YiB.rust-bundle) Rust extension to VS Code, to format and run test

### ⚒️ Scripts

Build app

```
cargo build --release --target=x86_64-unknown-linux-gnu
```

copy build app to root folder  project

```
cp target/x86_64-unknown-linux-gnu/release/lambda-rust ./bootstrap
```