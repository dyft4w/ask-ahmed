use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    embed_manifest(new_manifest("ask-ahmed")).expect("unable to embed manifest file");
    println!("cargo:rerun-if-changed=build.rs");

    windows_exe_info::icon::icon_ico("src/Ahmed.ico");
    println!("cargo:rerun-if-changed=src/Ahmed.ico");
    windows_exe_info::versioninfo::link_cargo_env();
}
