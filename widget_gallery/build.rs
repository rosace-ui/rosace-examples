//! Build script: scans `assets/` and generates the typed `assets` module
//! (included from `src/lib.rs`). Re-runs only when the asset tree changes.

fn main() {
    rosace_asset_codegen::generate("assets");
}
