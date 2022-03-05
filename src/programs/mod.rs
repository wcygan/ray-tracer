pub mod arch;

use clap::{ArgMatches, Command};
use image::{ImageBuffer, ImageFormat, Rgb};
use std::error::Error;
use std::path::Path;

pub trait MakeImage {
    fn subcommand() -> Command<'static>;
    fn make(matches: &ArgMatches) -> ImageBuffer<Rgb<u8>, Vec<u8>>;
}
