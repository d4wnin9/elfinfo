extern crate elfinfo;

use std::fs::File;


fn main() {
    let file = File::open("test-elf").unwrap();
    if let Err(e) = elfinfo::run(file) {
        eprintln!("Application Error: {}", e);
    }
}