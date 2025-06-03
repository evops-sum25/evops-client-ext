# EvOps / Client Extension

### UniFFI (Kotlin)

```shell
cargo rustc --crate-type=cdylib --lib --features=uniffi --release
```

```shell
cargo run --features=uniffi --release -- generate --language=kotlin --out-dir=target/ --library target/release/libevops_markdown.so
```

`target/uniffi/`

### Extism

```shell
cargo rustc --crate-type=cdylib --lib --features=extism --release --target=wasm32-unknown-unknown
```

`target/wasm32-unknown-unknown/release/evops_markdown.wasm`
