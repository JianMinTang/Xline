fn main() {
    tonic_build::configure()
        .compile(
            &["proto/kv.proto", "proto/rpc.proto", "proto/auth.proto"],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to build proto, error is {:?}", e));
}