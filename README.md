# EvOps / Client Extension

### UniFFI (Kotlin)

```shell
cargo ndk \
    --output-dir=../app/src/main/jniLibs \
    --target=armeabi-v7a \
    --target=arm64-v8a \
    --target=x86 \
    --target=x86_64 \
    build --lib --release

cargo run --package=evops-uniffi --release -- \
    generate --language=kotlin --out-dir=../app/src/main/kotlin/ --no-format --library target/release/libevops.so
```

`target/uniffi/`

### Extism

```shell
cargo build --package=evops-extism --release --target=wasm32-unknown-unknown
```

`target/wasm32-unknown-unknown/release/evops.wasm`
