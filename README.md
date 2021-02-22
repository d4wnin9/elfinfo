# elfinfo
binary static analysis tools like `readelf`, `checksec`.

## Installation
```install
cargo install --git https://github.com/d4wnin9/elfinfo.git --branch=main
```

## Usage
```sh
elfinfo [FLAGS] <filename>
```

## Flags
```txt
    -d, --dynamic            Display the dynamic section (if present)
        --help               Prints help information
    -h, --file-header        Display the ELF file header
    -l, --program-headers    Display the program headers
    -S, --section-headers    Display the sections' header
    -s, --syms               Display the symbol table
    -V, --version            Prints version information
```

## Args
```sh
<filename>    file name
```