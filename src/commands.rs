use crate::programs::arch::{Arch, ARCH};
use crate::MakeImage;
use clap::{Arg, ArgMatches, Command};
use image::ImageFormat;

use std::path::Path;

pub const FILENAME: &str = "out";
pub const WIDTH: &str = "width";
pub const HEIGHT: &str = "height";
pub const EXTENSION: &str = "ext";
pub const E: char = 'e';
pub const W: char = 'w';
pub const H: char = 'h';

pub fn run(matches: ArgMatches) {
    let image = match matches.subcommand() {
        Some((ARCH, sub_matches)) => Arch::make(sub_matches),
        _ => unreachable!(),
    };

    let ext = matches.value_of(EXTENSION).unwrap();
    let filename = format!("{}.{}", FILENAME, ext);
    image.save(filename).expect("error saving image");
}

pub fn get_subcommands() -> Vec<Command<'static>> {
    vec![Arch::subcommand()]
}

pub fn global_args() -> Vec<Arg<'static>> {
    vec![
        Arg::new(WIDTH)
            .long(WIDTH)
            .short(W)
            .help("The width of the canvas")
            .global(true)
            .required(false)
            .default_value("1000"),
        Arg::new(HEIGHT)
            .long(HEIGHT)
            .short(H)
            .help("The height of the canvas")
            .global(true)
            .required(false)
            .default_value("1000"),
        Arg::new(EXTENSION)
            .long(EXTENSION)
            .short(E)
            .help("The file extension to save (png, ppm, jpeg)")
            .global(true)
            .required(false)
            .default_value("ppm"),
    ]
}

pub fn width_and_height(matches: &ArgMatches) -> (u32, u32) {
    let w = matches.value_of(WIDTH).unwrap().parse::<u32>().unwrap();
    let h = matches.value_of(HEIGHT).unwrap().parse::<u32>().unwrap();
    (w, h)
}
