fn main() {
    tonic_build::compile_protos("proto/records.proto").unwrap();
}