use crate::api::weatherkit;
use chrono::{DateTime, Local};
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawBE},
    mono_font::{
        iso_8859_1::{FONT_10X20, FONT_4X6, FONT_5X7, FONT_5X8, FONT_6X9, FONT_9X15},
        MonoTextStyle,
    },
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
    Drawable,
};
use image::{DynamicImage, GenericImageView, RgbImage};
use rpi_led_panel::{Canvas, RGBMatrix, RGBMatrixConfig};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use tinytga::Tga;

pub fn draw(
    canvas: Canvas,
    matrix: &mut RGBMatrix,
    last_time_drawn: &mut DateTime<Local>,
) -> Option<Canvas> {
    // log::info!("drawing delay {}", Local::now() - *last_time_drawn);
    *last_time_drawn = Local::now();
    return Some(*matrix.update_on_vsync(Box::new(canvas)));
}

pub fn get_line_color(line: &str) -> Rgb888 {
    return match line {
        "U" => Rgb888::CSS_DARK_BLUE,
        "Bus" => Rgb888::CSS_DARK_VIOLET,
        "S" => Rgb888::CSS_DARK_GREEN,
        "Tram" => Rgb888::CSS_DARK_ORANGE,
        _ => Rgb888::WHITE,
    };
}

pub fn show_status_bar(
    led_func_data: crate::LedFuncData,
    mut canvas: Box<Canvas>,
) -> (Canvas, crate::LedFuncData) {
    let config: RGBMatrixConfig = crate::get_rgb_matrix_config();
    let rows = config.rows as i32;
    let cols = config.cols as i32;

    // let (_matrix, mut canvas) = RGBMatrix::new(config, 0).expect("Matrix initialization failed");

    let date_time = Local::now();
    let date = date_time.format("%d.%m").to_string();
    let time = date_time.format("%H:%M").to_string();

    canvas.fill(0, 0, 0);

    let first_text = Text::with_alignment(
        &date,
        Point::new(0, 7),
        MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
        Alignment::Left,
    )
    .draw(canvas.as_mut())
    .unwrap();

    Text::with_alignment(
        &time,
        Point::new(cols - 1, 7),
        MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
        Alignment::Right,
    )
    .draw(canvas.as_mut())
    .unwrap();

    Line::new(Point::new(0, 0), Point::new(cols, 0))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(canvas.as_mut())
        .unwrap();

    Line::new(Point::new(0, 9), Point::new(cols, 9))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(canvas.as_mut())
        .unwrap();

    return (
        *canvas,
        crate::LedFuncData {
            ..Default::default()
        },
    );
}
