use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/als/datapack.proto"], &["src/proto/"])?;
    Ok(())
}
