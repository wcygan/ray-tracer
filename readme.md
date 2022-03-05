# Ray-Tracer

A Rust implementation of [The Ray Tracer Challenge](http://raytracerchallenge.com/)

## Build & Run

### Installation via cargo

Make sure you have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed.

Install this project on any platform using Cargo:

```console
$ cargo install --git https://github.com/wcygan/ray-tracer
```

### How to run

```console
$ rtc
ray-tracer 0.1.0
Will C. <wcygan.io@gmail.com>
A Rust implementation of The Ray Tracer Challenge

USAGE:
    rtc [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -e, --ext <ext>          The file extension to save (png, ppm, jpeg) [default: ppm]
    -h, --height <height>    The height of the canvas [default: 1000]
        --help               Print help information
    -V, --version            Print version information
    -w, --width <width>      The width of the canvas [default: 1000]

SUBCOMMANDS:
    arch    Fires a bullet in an arching trajectory and writes the trajectory to the canvas
    help    Print this message or the help of the given subcommand(s)

```

### Running an example

```
$ rtc arch -e png
```

## Directory Tree
```
.
├── Cargo.toml
├── Cargo.lock
├── readme.md
└── src
    ├── commands.rs
    ├── programs
    │   ├── arch.rs
    │   └── mod.rs
    ├── lib
    │   ├── color.rs
    │   ├── tuple.rs
    │   ├── mod.rs
    │   └── canvas.rs
    └── main.rs
```