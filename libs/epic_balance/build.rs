use std::{env, fs};
use std::path::PathBuf;

#[cfg(feature = "serde")]
use pbjson_build;

fn main() -> Result<(), std::io::Error> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("proto");
    let proto_files = vec![root.join("balancing.proto")];

    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");

    prost_build::Config::new()
        .file_descriptor_set_path(&descriptor_path)
        .compile_protos(&proto_files, &[root])?;

    #[cfg(feature = "serde")]
    {
        let descriptor_set = std::fs::read(descriptor_path)?;

        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .build(&["."])?;
    }

    let build_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_files = fs::read_dir(&build_dir).unwrap();
    for file in build_files {
        let file = file.unwrap();
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        if file_name.starts_with("abepic.balancing.serde") {
            let file_content = fs::read_to_string(&file_path).unwrap();
            let new_content = file_content.replace("std::collections::HashMap", "indexmap::IndexMap");
            fs::write(&file_path, new_content).unwrap();
        }
    }

    Ok(())
}
