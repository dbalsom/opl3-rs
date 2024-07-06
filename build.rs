use std::env;
use std::path::PathBuf;

use bindgen::callbacks::{EnumVariantValue, ParseCallbacks};
use heck::ToUpperCamelCase;

#[derive(Debug)]
struct RenameCallbacks;

impl ParseCallbacks for RenameCallbacks {
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        Some(original_variant_name.to_upper_camel_case())
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        Some(original_item_name.to_upper_camel_case())
    }
}

fn main() {
    // The path to the header file
    let lib_path = "./src/nuked-opl3/";
    let header_path = "./src/nuked-opl3/opl3.h";

    // Tell cargo to rerun build.rs when the C library changes
    println!("cargo:rerun-if-changed={}", lib_path);

    let bindings_result = bindgen::Builder::default()
        .no_copy(".*")
        .header(header_path)
        .allowlist_function("OPL3.*")
        .parse_callbacks(Box::new(RenameCallbacks))
        .generate();

    match bindings_result {
        Ok(bindings) => {
            // Write the bindings to the $OUT_DIR/bindings.rs file
            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            if let Err(e) = bindings.write_to_file(out_path.join("bindings.rs")) {
                panic!("Failed to write bindings: {:?}", e);
            }
        }
        Err(e) => panic!("Failed to generate bindings: {:?}", e),
    }

    // Compile the C library
    cc::Build::new()
        .file("./src/nuked-opl3/opl3.c")
        .compile("opl3");

    // Link the compiled library
    println!("cargo:rustc-link-lib=static=opl3");

    println!("cargo:warning=Ran build.rs...");
}
