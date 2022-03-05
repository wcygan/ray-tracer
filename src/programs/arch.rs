use crate::commands::width_and_height;
use crate::lib::canvas::new_image_buffer;
use crate::programs::MakeImage;
use clap::{Arg, ArgMatches, Command};
use image::{ImageBuffer, Rgb};
use std::error::Error;

pub struct Arch {}

pub const ARCH: &str = "arch";

impl MakeImage for Arch {
    fn subcommand() -> Command<'static> {
        Command::new(ARCH).about("Creates a trajectory")
    }

    fn make(matches: &ArgMatches) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let (w, h) = width_and_height(&matches);
        let mut canvas = new_image_buffer(w, h);
        todo!()
    }
}
