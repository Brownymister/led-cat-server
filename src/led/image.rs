use embedded_graphics::{
    mono_font::{
        iso_8859_1::{FONT_10X20, FONT_4X6, FONT_5X7, FONT_5X8, FONT_6X9, FONT_9X15},
        MonoTextStyle,
    },
    pixelcolor::{Rgb888, Rgb565},
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
    Drawable,
    image::{Image, ImageRaw, ImageRawBE},
};
use image::{DynamicImage, GenericImageView, RgbImage};
use rpi_led_panel::{Canvas, RGBMatrix, RGBMatrixConfig};

pub fn show_image(path: &str) -> rpi_led_panel::Canvas {
    let config: RGBMatrixConfig = argh::from_env();
    let rows = config.rows;
    let cols = config.cols;
    let (mut matrix, mut canvas) = RGBMatrix::new(config, 0).expect("Matrix initialization failed");

    let img = image::open(path).expect("Failed to open image");

    // Convert the dynamic image to an RGB image buffer
    let rgb_img = match img {
        DynamicImage::ImageRgb8(buffer) => buffer,
        _ => img.to_rgb8(),
    };

    let image_data = rgb_img.into_raw();

    let image_data = ImageRawBE::<Rgb888>::new(&image_data, 64 as u32);
    let image = Image::new(
        &image_data,
        Point::new((cols / 2 - 64 / 2) as i32, (rows / 2 - 64 / 2) as i32),
    );

    image.draw(canvas.as_mut()).unwrap();

    return *canvas;
}
