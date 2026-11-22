fn main() {
    println!("cargo:rerun-if-changed=data/proxy_19041.bin");
    println!("cargo:rustc-check-cfg=cfg(has_proxy_blob)");
    if std::path::Path::new("data/proxy_19041.bin").is_file() {
        println!("cargo:rustc-cfg=has_proxy_blob");
    }
    xone_exports::emit_link_args("exports.txt");
}
