use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut out = File::create(Path::new(&out_dir).join("variables.rs")).unwrap();
    writeln!(out, "[");
    for i in 1..10000 {
        writeln!(out, "\"%{}\", ", i);
    }
    writeln!(out, "]");
}