# license-gen

CLI tool for generating `png` licenses. Currently works only on Linux (Maybe OSX?).

Written in Rust. Depends on `ImageMagick` v6.9.

```
license-gen 0.1.0
A license generator for Trials Licenses

USAGE:
    license-gen [FLAGS] --nickname <nickname> --output <output>

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --nickname <nickname>    Player's nickname
    -o, --output <output>        Output image path
```
