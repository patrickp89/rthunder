# rthunder
A GTK+ audio ripper frontend

## How to build it
Install the *Rust* toolchain (e.g. via [Rustup](https://www.rust-lang.org/tools/install)), the *GTK+* header files, and the header files for *libcddb2*. On Debian run:
```bash
# apt-get install libgtk-3-dev libcddb2-dev
```

Clone or download this repository and compile via Cargo:
```bash
$ cargo build
```
