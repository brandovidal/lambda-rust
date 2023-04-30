# cargo build --release --target=x86_64-unknown-linux-gnu &&
# cp target/x86_64-unknown-linux-gnu/release/lambda-rust ./bootstrap

# Path: build.sh
cargo build --release --target=x86_64-unknown-linux-musl &&
cp target/x86_64-unknown-linux-musl/release/lambda-rust ./bootstrap