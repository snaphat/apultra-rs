extern crate cc;
use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

fn read_file<S: AsRef<str>>(path: S) -> String {
    let mut file = File::open(path.as_ref()).expect("Unable to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Unable to read file");
    String::from_utf8_lossy(&data).chars().collect()
}

fn write_file<S: AsRef<str>>(path: S, data: String) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path.as_ref())
        .expect("Unable to open file");

    file.write(data.as_bytes()).expect("Unable to write file");
}

fn main() {
    let dir = "apultra/src";
    let apultra_c = dir.to_owned() + "/apultra.c";
    let apultra_mod_c =
        env::var("OUT_DIR").expect("Failed to get OUT_DIR environment variable") + "/apultra.mod.c";

    // Read the c file and replace name so library can be linked.
    let data = read_file(apultra_c).replace("int main", "int __main");
    // Write modified data to file.
    write_file(&apultra_mod_c, data);

    cc::Build::new()
        .file(apultra_mod_c)
        .file(dir.to_owned() + "/expand.c")
        .file(dir.to_owned() + "/matchfinder.c")
        .file(dir.to_owned() + "/shrink.c")
        .file(dir.to_owned() + "/libdivsufsort/lib/divsufsort.c")
        .file(dir.to_owned() + "/libdivsufsort/lib/divsufsort_utils.c")
        .file(dir.to_owned() + "/libdivsufsort/lib/sssort.c")
        .file(dir.to_owned() + "/libdivsufsort/lib/trsort.c")
        .include(dir.to_owned() + "/")
        .include(dir.to_owned() + "/libdivsufsort/include")
        .compile("libfoo.a");
}
