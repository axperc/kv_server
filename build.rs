use std::path;
fn main() {
    let mut config = prost_build::Config::new();
    config
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize,PartialOrd)]",
        )
        .out_dir(path::PathBuf::from("src/pb"))
        .compile_protos(&["src/pb/abi.proto"], &["."])
        .unwrap();
}
