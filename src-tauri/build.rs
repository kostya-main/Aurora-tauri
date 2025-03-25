fn main() {
    if std::env::consts::OS == "windows" {
        std::env::set_var("PROTOC", "../protoc/bin/protoc.exe");
    }
    tonic_build::configure()
        .build_server(false)
        .type_attribute(
            "AuthResponse",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "ServersResponse",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "ServerConfig",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute("Server", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_protos(
            &["../node_modules/@aurora-launcher/proto/packages/proto/main.proto"],
            &["../node_modules/@aurora-launcher/proto/packages/proto"],
        )
        .unwrap();
    tauri_build::build()
}
