# rthunder
[![Build Status](https://travis-ci.org/patrickp89/rthunder.svg?branch=master)](https://travis-ci.org/patrickp89/rthunder)

A GTK+ audio ripper frontend.

## How to build it
Install [Rust](https://www.rust-lang.org/) and Cargo (e.g. via [Rustup](https://www.rust-lang.org/tools/install)).
You'll also need build essentials (e.g. *gcc* and *make*) as well as a couple of libraries, and their header files. On
Debian run:
```bash
# apt-get install curl build-essential libgtk-3-dev libcddb2-dev libcdio-paranoia-dev libiso9660-dev libudf-dev
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone or download this repository and build via Cargo:
```bash
$ cargo build
```

## Troubleshooting

#### Undefined references when linking
If Cargo displays an error message like "undefined reference to xyz / error: ld returned 1 exit status", your linker
is not able to find (one or more) referenced functions. Here is what you can do:
* Make sure that you have installed the libraries mentioned above
* If the path where those libraries were installed differs, create a symlink, e.g.
```bash
$ ln -s /path/to/your/libs/libcddb.so /usr/lib/x86_64-linux-gnu/libcddb.so
```
* As a last resort you can change the linker's search path in the [Makefile](native/Makefile) by altering the "-L" flag
