// netmap doesn't provide these functions as a library, so we cheat, to save porting them manually
// to Rust. This is a very ugly hack.
extern crate cc;
#[cfg(feature = "libnetmap")]
extern crate pkg_config;
use std::env;
use std::io::prelude::*;
use std::fs;
use std::path::Path;

fn main() {

    if let Some(_) = env::var_os("CARGO_FEATURE_NETMAP_WITH_LIBS") {
        let out_dir = env::var("OUT_DIR").unwrap();
        let tmp_path = Path::new(&out_dir).join("netmap.c");
        let mut tmp = fs::File::create(&tmp_path).unwrap();

        tmp.write_all(b"#include <sys/time.h>\n\
                        #include <errno.h>\n\
                        typedef unsigned int u_int;
                        typedef unsigned long u_long;
                        typedef unsigned char u_char;
                        #include <net/netmap_user.h>\n").unwrap();
        cc::Build::new()
            .file(&tmp_path)
            .define("NETMAP_WITH_LIBS", None)
            .define("static", Some(""))
            .define("inline", Some(""))
            .include("netmap/sys")
            .compile("librust_netmap_user.a");
        fs::remove_file(&tmp_path).unwrap();
        println!("cargo:rustc-link-lib=rust_netmap_user");
    }
    if let Some(_) = env::var_os("CARGO_FEATURE_LIBNETMAP")  {
        println!("cargo:rustc-link-search=/usr/local/lib/");
        println!("cargo:rustc-link-lib=netmap");

        //let out_dir = env::var("OUT_DIR").unwrap();
        //fs::copy("/usr/local/lib/libnetmap.a", out_dir + "/librust_netmap_user.a").unwrap();
    }
}
