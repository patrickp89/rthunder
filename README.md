# rthunder
A GTK+ audio ripper frontend

## How to build it
Install the *Rust* toolchain (e.g. via [Rustup](https://www.rust-lang.org/tools/install)), as well as *gcc*, and *make*.
Install the *GTK+*, *libcddb2*, and *libcdio-paranoia* libraries as well as their header files. On Debian run:
```bash
# apt-get install build-essential libgtk-3-dev libcddb2-dev libcdio-paranoia-dev
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
* If the path where those libraries were installed differ, create a symlink, e.g.
```bash
$ ln -s /path/to/your/libs/libcddb.so /usr/lib/x86_64-linux-gnu/libcddb.so
```
* As a last resort you can change the linker path in  linker option to the [build.rs](build.rs) file
