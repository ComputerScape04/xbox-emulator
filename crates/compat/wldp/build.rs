fn check_data_blobs() {
    println!("cargo:rustc-check-cfg=cfg(has_wldp_blobs)");
    let src_data = std::path::Path::new("data");
    let guids = [
        "activity_provider_guid.bin",
        "state_provider_guid.bin",
        "watermark_provider_guid.bin",
    ];
    let descs = [
        "wdac_event_desc.bin",
        "blocklist_event_desc.bin",
        "retail_unlocked_event_desc.bin",
        "hsti_failure_event_desc.bin",
        "production_event_desc.bin",
    ];
    let metas = [
        "activity_provider_meta.bin",
        "state_provider_meta.bin",
        "wdac_event_meta.bin",
        "blocklist_event_meta.bin",
        "watermark_provider_meta.bin",
        "retail_unlocked_event_meta.bin",
        "hsti_failure_event_meta.bin",
        "production_event_meta.bin",
    ];
    if guids.iter().chain(descs.iter()).chain(metas.iter()).all(|n| src_data.join(n).is_file()) {
        println!("cargo:rustc-cfg=has_wldp_blobs");
    }
}

fn main() {
    println!("cargo:rerun-if-changed=exports.txt");
    println!("cargo:rerun-if-changed=data");
    check_data_blobs();
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").map(|e| e == "msvc").unwrap_or(false);
    if !msvc {
        return;
    }
    let text = std::fs::read_to_string("exports.txt").expect("exports.txt");
    for e in xone_exports::parse(&text) {
        if e.status == "missing" {
            continue;
        }
        match e.note.strip_prefix("PE forwarder to ").and_then(|r| r.split(';').next()) {
            Some(target) => println!("cargo:rustc-cdylib-link-arg=/EXPORT:{}={},@{}", e.name, target.trim(), e.ordinal),
            None => println!("cargo:rustc-cdylib-link-arg=/EXPORT:{},@{}", e.name, e.ordinal),
        }
    }
}
