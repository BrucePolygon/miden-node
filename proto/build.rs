use miette::IntoDiagnostic;
use prost::Message;
use std::env;
use std::{fs, path::PathBuf};

fn main() -> miette::Result<()> {
    // Compute the directory of the `proto` definitions
    let cwd: PathBuf = env::current_dir().into_diagnostic()?;
    let proto_dir: PathBuf = cwd.join("proto");

    // Compute the compiler's target file path.
    let out = env::var("OUT_DIR").into_diagnostic()?;
    let file_descriptor_path = PathBuf::from(out).join("file_descriptor_set.bin");

    // Compile the proto file for all servers APIs
    let protos = &[
        proto_dir.join("block_producer.proto"),
        proto_dir.join("store.proto"),
        proto_dir.join("rpc.proto"),
    ];
    let includes = &[proto_dir];
    let file_descriptors = protox::compile(protos, includes)?;
    fs::write(&file_descriptor_path, file_descriptors.encode_to_vec()).into_diagnostic()?;

    // Generate the stub of the user facing server from its proto file
    tonic_build::configure()
        .file_descriptor_set_path(&file_descriptor_path)
        .type_attribute(".", "#[derive(Eq, PartialOrd, Ord, Hash)]")
        .skip_protoc_run()
        .out_dir("src/generated")
        .compile(protos, includes)
        .into_diagnostic()?;

    Ok(())
}
