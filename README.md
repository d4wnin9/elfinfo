# elfinfo
binary static analysis tool like `readelf`, `checksec`.

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
    -h, --file-header        Display the ELF file header
    -V, --version            Prints version information
```

## Args
```sh
<filename>    file name
```

## 参考
- https://github.com/Drumato/elf-utilities
- [ELF入門 - 情弱ログ](https://sugawarayusuke.hatenablog.com/entry/2017/04/09/213133)
- [readelf -hの簡易版･省略版を作成するミニ記事 - Explore cs in depth!](https://drumato.hatenablog.com/entry/2019/04/17/080816)
- [ELF Formatについて](http://caspar.hazymoon.jp/OpenBSD/annex/elf.html)
- [ELF Header](https://refspecs.linuxfoundation.org/elf/gabi4+/ch4.eheader.html)
- [ELF Header](https://docs.oracle.com/cd/E19957-01/806-0641/chapter6-43405/index.html)
- [ELFファイルを作る part7](https://warabanshi.hatenablog.com/entry/2013/05/18/231628)