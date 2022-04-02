pub mod arch;

use clap::{ArgMatches, Command};
use image::{ImageBuffer, Rgb};

pub trait MakeImage {
    fn subcommand() -> Command<'static>;
    fn make(matches: &ArgMatches) -> ImageBuffer<Rgb<u8>, Vec<u8>>;
}
