// #![warn(clippy::implicit_return)]
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawBE},
    mono_font::{
        iso_8859_1::{FONT_10X20, FONT_4X6, FONT_5X7, FONT_5X8, FONT_6X9, FONT_9X15},
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
    Drawable,
};
use rand::seq::SliceRandom;
use rand::Rng;
use rpi_led_panel::Canvas;
use std::f32::consts::PI;

pub async fn show_fireplace(
    led_func_data: crate::LedFuncData,
    mut canvas: Box<Canvas>,
) -> (rpi_led_panel::Canvas, crate::LedFuncData) {
    let config: rpi_led_panel::RGBMatrixConfig = crate::get_rgb_matrix_config();
    let rows = config.rows as i32;
    let cols = config.cols as i32;
    let mut fireplace_data = led_func_data.fireplace_data.clone().unwrap();
    fireplace_data.dark_rows.clear();
    fireplace_data.orange_rows.clear();
    fireplace_data.yelllow_rows.clear();

    if fireplace_data.time == 0.0 {
        println!("ran");
        for x in 0..cols {
            let dark_line = get_random_line_posi(x, 15, 35, 10, 40);
            fireplace_data.dark_rows.push(dark_line.clone());

            let orange_line = get_random_line_posi(x, 20, 40, 10, 30);
            fireplace_data.orange_rows.push(orange_line.clone());

            let yellow_line = get_random_line_posi(x, 32, 64, 10, 15);
            fireplace_data.yelllow_rows.push(yellow_line.clone());
        }
        fireplace_data.time = 0.1;
    } else {
        println!("ran here");
        for line in led_func_data.fireplace_data.clone().unwrap().dark_rows {
            draw_line(
                line,
                &mut fireplace_data,
                canvas.as_mut(),
                Rgb888::CSS_DARK_RED,
                0,
                40,
            );
        }
        for line in led_func_data.fireplace_data.clone().unwrap().orange_rows {
            draw_line(
                line,
                &mut fireplace_data,
                canvas.as_mut(),
                Rgb888::CSS_ORANGE,
                16,
                55,
            );
        }
        for line in led_func_data.fireplace_data.clone().unwrap().yelllow_rows {
            draw_line(
                line,
                &mut fireplace_data,
                canvas.as_mut(),
                Rgb888::CSS_YELLOW,
                40,
                64,
            )
        }
    }

    return (
        *canvas,
        crate::LedFuncData {
            fireplace_data: Some(fireplace_data),
            ..Default::default()
        },
    );
}

fn get_random_line_posi(x: i32, y_start: i32, y_end: i32, min_len: i32, max_len: i32) -> Line {
    let mut rng = rand::thread_rng();
    let y_start = rng.gen_range(y_start..y_end);
    let y_end = y_start + rng.gen_range(min_len..max_len); // Adjust the range for different line lengths
    let line = Line::new(Point::new(x, y_start), Point::new(x, y_end));
    return line;
}

fn draw_line(
    line: Line,
    fireplace_data: &mut crate::FireplaceData,
    canvas: &mut Canvas,
    color: Rgb888,
    upper_border: i32,
    buttom_border: i32,
) {
    let x = line.start.x;
    let mut rng = rand::thread_rng();

    let mut y_start = line.start.y;
    let mut y_end = line.end.y;
    if rng.gen_range(0..2) == 0 {
        y_start = line.start.y - 2;
        y_end = line.end.y - 2;
    } else {
        y_start = line.start.y + 2;
        y_end = line.end.y + 2;
    }

    if y_start < upper_border || y_end < upper_border {
        y_start = line.start.y + 10;
        y_end = line.end.y + 10;
    }

    if y_end > buttom_border || y_start > buttom_border {
        y_start = line.start.y - 1;
        y_end = line.end.y - 1;
    }
    let line = Line::new(Point::new(x, y_start), Point::new(x, y_end));

    if color == Rgb888::CSS_DARK_RED {
        fireplace_data.dark_rows.push(line.clone());
    }
    if color == Rgb888::CSS_ORANGE {
        fireplace_data.orange_rows.push(line.clone());
    }
    if color == Rgb888::YELLOW {
        fireplace_data.yelllow_rows.push(line.clone());
    }
    line.into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(canvas)
        .expect("Failed to draw line");
}
