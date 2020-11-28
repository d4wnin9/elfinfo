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
今はまだheaderのみ

## Flags
```sh
    --help               Prints help information
-h, --file-header        Display the ELF file header
-l, --program-headers    Display the program headers
-V, --version            Prints version information

```

## Args
```sh
<filename>    file name
```