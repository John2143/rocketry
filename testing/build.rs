// build.rs

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dest_path)
        .unwrap();

    write!(file, "const SINS: [i16; 256] = [\n").unwrap();
    for i in 0..=0xFF {
        let f = (i as f32) * (2.0 * 3.1415926) / 256.0;
        let new_i = (f.sin() * 256.0).floor() as i16;
        write!(file, "{},\n", new_i).unwrap();
    }

    write!(file, "];").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
