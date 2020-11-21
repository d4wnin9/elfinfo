extern crate elfinfo;

fn main() {
    let filename = "test-elf";
    if let Err(e) = elfinfo::run(filename) {
        eprintln!("Application Error: {}", e);
    };
}