use crate::commands::{get_subcommands, global_args, run};
use crate::programs::MakeImage;
use clap::{AppSettings, Arg, Command};
use image::{ImageBuffer, ImageFormat, Rgb};
use std::path::Path;

mod commands;
mod lib;
mod programs;

fn main() {
    let matches = Command::new("ray-tracer")
        .version("0.1.0")
        .author("Will C. <wcygan.io@gmail.com>")
        .about("A Rust implementation of The Ray Tracer Challenge")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .args(global_args())
        .subcommands(get_subcommands())
        .get_matches();

    run(matches)
}
