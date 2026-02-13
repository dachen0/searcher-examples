use tonic_build::configure;

fn main() {
    configure()
        .compile(
            &[
                "protos/proto/auth.proto",
                "protos/proto/bundle.proto",
                "protos/proto/packet.proto",
                "protos/proto/searcher.proto",
                "protos/proto/shared.proto",
            ],
            &["protos/proto"],
        )
        .unwrap();
}
