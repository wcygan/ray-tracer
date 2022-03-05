use crate::commands::width_and_height;
use crate::lib::canvas::{
    convert_to_bottom_up_coordinates, neighbors, new_image_buffer, point_is_in_rectangle,
};
use crate::lib::color::{color, color_into_rgb};
use crate::lib::tuple::{add_tup, mul_tup, normalize, point, sub_tup, vector};
use crate::programs::MakeImage;
use clap::{Arg, ArgMatches, Command};
use image::{ImageBuffer, Rgb};

pub struct Arch {}

pub const ARCH: &str = "arch";

pub const XL: &str = "x";
pub const X: char = 'x';
pub const YL: &str = "y";
pub const Y: char = 'y';
pub const ML: &str = "m";
pub const M: char = 'm';

impl MakeImage for Arch {
    fn subcommand() -> Command<'static> {
        Command::new(ARCH)
            .about(
                "Fires a bullet in an arching trajectory and writes the trajectory to the canvas",
            )
            .arg(
                Arg::new(XL)
                    .long(XL)
                    .short(X)
                    .help("The X component of the initial velocity")
                    .required(false)
                    .default_value("5"),
            )
            .arg(
                Arg::new(YL)
                    .long(YL)
                    .short(Y)
                    .help("The Y component of the initial velocity")
                    .required(false)
                    .default_value("15"),
            )
            .arg(
                Arg::new(ML)
                    .long(ML)
                    .short(M)
                    .help("The magnitude to fire the trajectory 'bullet' at")
                    .required(false)
                    .default_value("14.5"),
            )
    }

    fn make(matches: &ArgMatches) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let (mut w, mut h) = width_and_height(matches);
        let (mut x, mut y) = x_and_y(matches);
        let m = magnitude(matches);
        let color = color_into_rgb(color(0.1, 0.8, 0.2));
        let mut canvas = new_image_buffer(w, h);

        let mut velocity = mul_tup(normalize(vector(x as f64, y as f64, 0.0)), m);
        let mut start = point(10.0, 10.0, 0.0);
        let drag = vector(-0.01, -0.1, 0.0);

        while start.1 >= 0.0 {
            let (x, y) = convert_to_bottom_up_coordinates(
                start.0.round() as i32,
                start.1.round() as i32,
                w as i32,
                h as i32,
            );

            if !point_is_in_rectangle(x, y, w as i32, h as i32) {
                break;
            }

            for pt in neighbors(x, y, w as i32, h as i32, 2) {
                canvas.put_pixel(pt.0 as u32, pt.1 as u32, color);
            }

            canvas.put_pixel(x as u32, y as u32, color);
            velocity = add_tup(velocity, drag);
            start = add_tup(start, velocity);
        }

        canvas
    }
}

fn x_and_y(matches: &ArgMatches) -> (u32, u32) {
    let x = matches.value_of(XL).unwrap().parse::<u32>().unwrap();
    let y = matches.value_of(YL).unwrap().parse::<u32>().unwrap();
    (x, y)
}

fn magnitude(matches: &ArgMatches) -> f64 {
    matches.value_of(ML).unwrap().parse::<f64>().unwrap()
}
