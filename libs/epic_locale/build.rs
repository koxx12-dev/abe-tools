use std::env;
use std::path::PathBuf;

#[cfg(feature = "serde")]
use pbjson_build;

fn main() -> Result<(), std::io::Error> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("proto");
    let proto_files = vec![root.join("locale.proto")];

    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");

    prost_build::Config::new()
        .file_descriptor_set_path(&descriptor_path)
        // .compile_well_known_types()
        // .extern_path(".google.protobuf", "::pbjson_types")
        .compile_protos(&proto_files, &[root])?;

    #[cfg(feature = "serde")]
    {
        let descriptor_set = std::fs::read(descriptor_path)?;

        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .build(&[".abepic.locale"])?;
    }
    
    Ok(())
}
