extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // bluez has the following dependencies for mesh_cfgclient:
    // tools_mesh_cfgclient_LDADD = lib/libbluetooth-internal.la src/libshared-ell.la \
    // $(ell_ldadd) -ljson-c -lreadline
    // Link json-c and readline should be installed in the system
    pkg_config::Config::new().probe("json-c").unwrap();
    pkg_config::Config::new().probe("readline").unwrap();

    // bluetooth and ell need to be compiled in order to be linked
    // pkg_config::Config::new().probe("bluetooth").unwrap();
    // libbluetooth-internal.la
    // 

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}