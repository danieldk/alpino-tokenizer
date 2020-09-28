fn main() {
    prost_build::compile_protos(&["proto/transducer.proto"], &["proto/"]).unwrap();
}
