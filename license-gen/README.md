# license-gen

CLI tool for generating `png` licenses. Currently works only on Linux (Maybe OSX?).

Written in Rust. Depends on `ImageMagick` v6.9.

```
license-gen 0.4.0
A license generator for Trials Licenses

USAGE:
    license-gen [OPTIONS] --nickname <nickname>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --advanced <advanced>...               Advanced license bikes
    -b, --basic <basic>...                     Basic license bikes
    -n, --nickname <nickname>                  Player's nickname
    -o, --output <output>                      Output image path [default: $nickname.jpg]
    -p, --profile-picture <profile-picture>    Path to the profile picture
```
