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

o 

run script

```
sudo chmod gu+x build.sh && ./build.sh
````

### Example

Request get pizza by name
```
curl -i https://8jcf5ibyi9.execute-api.sa-east-1.amazonaws.com/dev/pizza/veggie/price
````