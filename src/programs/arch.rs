use crate::commands::width_and_height;
use crate::lib::canvas::new_image_buffer;
use crate::lib::tuple::{point, vector};
use crate::programs::MakeImage;
use clap::{Arg, ArgMatches, Command};
use image::{ImageBuffer, Rgb};

pub struct Arch {}

pub const ARCH: &str = "arch";

pub const XL: &str = "x";
pub const X: char = 'x';
pub const YL: &str = "y";
pub const Y: char = 'y';

impl MakeImage for Arch {
    fn subcommand() -> Command<'static> {
        Command::new(ARCH)
            .about("Creates a trajectory")
            .arg(
                Arg::new(XL)
                    .long(XL)
                    .short(X)
                    .help("The X component of the initial velocity")
                    .required(false)
                    .default_value("1"),
            )
            .arg(
                Arg::new(YL)
                    .long(YL)
                    .short(Y)
                    .help("The Y component of the initial velocity")
                    .required(false)
                    .default_value("1"),
            )
    }

    fn make(matches: &ArgMatches) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let (mut w, mut h) = width_and_height(matches);
        let (mut x, mut y) = x_and_y(matches);
        let mut velocity = vector(x as f64, y as f64, 0.0);
        let mut start = point(0.0, 0.0, 0.0);
        let mut canvas = new_image_buffer(w, h);

        canvas
    }
}

fn x_and_y(matches: &ArgMatches) -> (u32, u32) {
    let x = matches.value_of(XL).unwrap().parse::<u32>().unwrap();
    let y = matches.value_of(YL).unwrap().parse::<u32>().unwrap();
    (x, y)
}
