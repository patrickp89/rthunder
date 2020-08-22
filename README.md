# rthunder
[![Build Status](https://travis-ci.org/patrickp89/rthunder.svg?branch=master)](https://travis-ci.org/patrickp89/rthunder)

A GTK+ audio ripper frontend.

## How to build it
Install [Rust](https://www.rust-lang.org/) and Cargo (e.g. via [Rustup](https://www.rust-lang.org/tools/install)).
You'll also need build essentials (e.g. *gcc* and *make*) as well as a couple of libraries, and their header files. On
Debian run:
```bash
# apt-get install build-essential libgtk-3-dev libcddb2-dev libcdio-paranoia-dev libiso9660-dev libudf-dev
# apt-get install libcurl4-openssl-dev libelf-dev libdw-dev binutils-dev libsoup2.4-dev libxtst-dev at-spi2-core libxdo-dev libmount-dev
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone or download this repository and build via Cargo:
```bash
$ cargo build
```
