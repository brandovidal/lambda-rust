# Rust on AWS Lambda using Serverless framework

### 📌 Pre requisites

[Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) in your system.

Install this target needed to build

```
rustup target add x86_64-unknown-linux-musl
```

[Vs Code Extension](https://marketplace.visualstudio.com/items?itemName=1YiB.rust-bundle) Rust extension to VS Code, to format and run test.

### ⚒️ Scripts

Build App

```
pnpm run build
```

Deploy app

```
pnpm run deploy
```

Print variables

```
pnpm run print
```

Remove package and generate

```
pnpm run clean
```

### ⌛ Requests

Request to find non-existent pizza

```
curl -i  https://3gaz7pa0g9.execute-api.us-east-1.amazonaws.com/dev/pizza/xxx/price
```

Request to find pizza by name

```
curl -i  https://3gaz7pa0g9.execute-api.us-east-1.amazonaws.com/dev/pizza/veggie/price
```
