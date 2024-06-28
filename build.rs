use std::env;
use std::path::PathBuf;
use heck::ToUpperCamelCase;
use bindgen::callbacks::{EnumVariantValue, ParseCallbacks};

#[derive(Debug)]
struct RenameCallbacks;

impl ParseCallbacks for RenameCallbacks {
    fn enum_variant_name(&self, _enum_name: Option<&str>, original_variant_name: &str, _variant_value: EnumVariantValue) -> Option<String> {
        Some(original_variant_name.to_upper_camel_case())
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        Some(original_item_name.to_upper_camel_case())
    }
}

fn main() {
    // The path to the header file
    let header_path = "./src/nuked-opl3/opl3.h";

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", header_path);

    let bindings_result = bindgen::Builder::default()
        .header(header_path)
        .allowlist_function("OPL3.*")
        .parse_callbacks(Box::new(RenameCallbacks))
        .generate();

    match bindings_result {
        Ok(bindings) => {
            println!("Bindings built successfully!");

            // Write the bindings to the $OUT_DIR/bindings.rs file
            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            println!("Writing bindings to {:?}", out_path.join("bindings.rs"));
            match bindings
                .write_to_file(out_path.join("bindings.rs")) {

                Ok(_) => println!("Bindings written successfully!"),
                Err(e) => panic!("Failed to write bindings: {:?}", e),
            }
        },
        Err(e) => panic!("Failed to generate bindings: {:?}", e),
    }

    // Compile the C library
    cc::Build::new()
        .file("./src/nuked-opl3/opl3.c")
        .compile("opl3");
}