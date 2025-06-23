fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["../../proto/evops/ext/v1/ext.proto"], &["../../proto/"])?;
    Ok(())
}
