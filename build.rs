use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use io::StoreMetadata;

fn main() {
    WasmBuilder::with_meta(StoreMetadata::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}