use crate::programs::arch::{Arch, ARCH};
use crate::MakeImage;
use clap::{Arg, ArgMatches, Command};
use image::{ImageBuffer, ImageFormat, Rgb};
use std::env::Args;
use std::path::Path;

pub const FILENAME: &str = "out.ppm";
pub const WIDTH: &str = "width";
pub const HEIGHT: &str = "height";
pub const W: char = 'w';
pub const H: char = 'h';

pub fn run(matches: ArgMatches) {
    let image = match matches.subcommand() {
        Some((ARCH, sub_matches)) => Arch::make(sub_matches),
        _ => unreachable!(),
    };

    image
        .save_with_format(&Path::new(FILENAME), ImageFormat::Pnm)
        .expect("file failed to save")
}

pub fn get_subcommands() -> Vec<Command<'static>> {
    vec![Arch::subcommand()]
}

pub fn global_args() -> Vec<Arg<'static>> {
    vec![
        Arg::new(WIDTH)
            .long(WIDTH)
            .short(W)
            .global(true)
            .required(false)
            .default_value("1000"),
        Arg::new(HEIGHT)
            .long(HEIGHT)
            .short(H)
            .global(true)
            .required(false)
            .default_value("1000"),
    ]
}

pub fn width_and_height(matches: &ArgMatches) -> (u32, u32) {
    let w = matches.value_of(WIDTH).unwrap().parse::<u32>().unwrap();
    let h = matches.value_of(HEIGHT).unwrap().parse::<u32>().unwrap();
    (w, h)
}
