#[cfg(feature = "protobuf")]
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(
        &["../proto/evops/markdown/v1/markdown.proto"],
        &["../proto/"],
    )?;
    Ok(())
}

#[cfg(not(feature = "protobuf"))]
fn main() {}
