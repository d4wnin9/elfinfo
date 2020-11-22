extern crate elfinfo;
#[macro_use]
extern crate clap;

use clap::{App, Arg};


fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())

        .arg(Arg::with_name("header")
            .help("display header")
            .short("h")
            .long("file-header")
        )

        .arg(Arg::with_name("filename")
            .help("file name")
            .required(true)
        );

    let args = app.get_matches();

    let filename: &str = args.value_of("filename").unwrap();

    if args.is_present("header") {
        if let Err(e) = elfinfo::run(filename) {
            eprintln!("Application Error: {}", e);
        };
    } else {
        println!("Bye, brother, see you later.");
    }
}