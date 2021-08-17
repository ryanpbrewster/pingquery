fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.out_dir("src/proto");
    config.type_attribute(
        ".",
        "#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = \"camelCase\")]",
    );
    config.field_attribute(".", "#[serde(default)]");
    config.compile_protos(&["proto/api.proto"], &["proto"])?;
    Ok(())
}
