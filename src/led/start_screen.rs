use chrono::{DateTime, Local};
use embedded_graphics::{
    mono_font::{
        iso_8859_1::{FONT_10X20, FONT_4X6, FONT_5X7, FONT_5X8, FONT_6X9, FONT_9X15},
        MonoTextStyle,
    },
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
    Drawable,
};
use rpi_led_panel::{Canvas, RGBMatrix, RGBMatrixConfig};

pub fn start_srceen() -> (rpi_led_panel::Canvas, crate::LedFuncData) {
    let config: RGBMatrixConfig = crate::get_rgb_matrix_config();
    let rows = config.rows as i32;
    let cols = config.cols as i32;

    let (_matrix, mut canvas) = RGBMatrix::new(config, 0).expect("matrix initialization failed");

    let circle = {
        let thin_stroke = PrimitiveStyle::with_stroke(Rgb888::CSS_GRAY, 1);
        Circle::with_center(
            Point::new(rows / 2 - 1, cols / 2 - 1),
            rows.min(cols) as u32 - 2,
        )
        .into_styled(thin_stroke)
    };

    // let auth_num_str = &auth_num_value.clone().to_string();
    let text = Text::with_alignment(
        "Led Cat",
        Point::new(cols / 2, rows / 2),
        MonoTextStyle::new(&FONT_9X15, Rgb888::CYAN),
        Alignment::Center,
    );

    canvas.fill(0, 0, 0);
    circle.draw(canvas.as_mut()).unwrap();
    text.draw(canvas.as_mut()).unwrap();
    return (*canvas, Default::default());
}
