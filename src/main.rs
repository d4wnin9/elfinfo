extern crate elfinfo;
#[macro_use]
extern crate clap;

use clap::{App, Arg};


fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())

        .arg(Arg::with_name("hdr")
            .help("Display the ELF file header")
            .short("h")
            .long("file-header")
        )

        .arg(Arg::with_name("phdr")
            .help("Display the program headers")
            .short("l")
            .long("program-headers")
            .alias("segments")
        )

        .arg(Arg::with_name("shdr")
            .help("Display the sections' header")
            .short("S")
            .long("section-headers")
            .alias("sections")
        )

        .arg(Arg::with_name("sym")
            .help("Display the symbol table")
            .short("s")
            .long("syms")
            .alias("symbols")
        )

        .arg(Arg::with_name("dyn")
            .help("Display the dynamic section (if present)")
            .short("d")
            .long("dynamic")
        )

        .arg(Arg::with_name("checksec")
            .help("Display properties of executables")
            .short("c")
            .long("checksec")
        )

        .arg(Arg::with_name("filename")
            .help("file name")
            .required(true)
        );

    let args = app.get_matches();

    let filename: &str = args.value_of("filename").unwrap();

    if args.is_present("hdr") {
        if let Err(e) = elfinfo::run(filename, "hdr") {
            eprintln!("Application Error: {}", e);
        };
    } else if args.is_present("phdr") {
        if let Err(e) = elfinfo::run(filename, "phdr") {
            eprintln!("Application Error: {}", e);
        };
    } else if args.is_present("shdr") {
        if let Err(e) = elfinfo::run(filename, "shdr") {
            eprintln!("Application Error: {}", e);
        };
    } else if args.is_present("sym") {
        if let Err(e) = elfinfo::run(filename, "sym") {
            eprintln!("Application Error: {}", e);
        };
    } else if args.is_present("dyn") {
        if let Err(e) = elfinfo::run(filename, "dyn") {
            eprintln!("Application Error: {}", e);
        };
    } else if args.is_present("checksec") {
        if let Err(e) = elfinfo::run(filename, "checksec") {
            eprintln!("Application Error: {}", e);
        };
    } else {
        println!("Bye, brother, see you later.");
    }
}