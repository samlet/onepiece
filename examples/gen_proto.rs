use protoc_rust::Customize;

// ⊕ [rust-protobuf/protoc-rust at master · stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf/tree/master/protoc-rust)
fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        // input: &["protos/a.proto", "protos/b.proto"],
        input: &["protos/simple.proto", "protos/addressbook.proto"],
        includes: &["protos"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
