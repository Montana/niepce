extern crate pkg_config;

fn main() {
    println!("cargo:rustc-link-lib=niepce_lib");
    println!("cargo:rustc-link-lib=ncr");
    println!("cargo:rustc-link-lib=fwk");
    println!("cargo:rustc-link-lib=rtengine");
    if let Ok(asan) = std::env::var("ASAN_LIBS") {
        println!("cargo:rustc-link-lib={asan}");
    }

    // This is expected to be set by the mesonb.build.
    if let Ok(libpath) = std::env::var("NIEPCE_LIB_PATH") {
        libpath.split(':').for_each(|s| {
            println!("cargo:rustc-link-search={s}");
        });
    }

    // Direct dependencies by the C++ code.
    pkg_config::Config::new()
        .atleast_version("4.4.0")
        .probe("gtkmm-4.0")
        .unwrap();
    pkg_config::Config::new()
        .atleast_version("1.0.0")
        .probe("shumate-1.0")
        .unwrap();
}
