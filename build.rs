#![no_std]
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;
use io::StoreMetadata;
use scale_info::prelude::vec;

fn main() {
    WasmBuilder::with_meta(StoreMetadata::repr())
        .exclude_features(vec!["binary-vendor"])
        .build();
}