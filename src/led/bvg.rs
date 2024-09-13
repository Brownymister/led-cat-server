use crate::led::led::get_line_color;
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

pub async fn show_bvg_timetable(
    led_func_data: crate::LedFuncData,
    mut canvas: Box<Canvas>,
) -> (Canvas, crate::LedFuncData) {
    let config = crate::get_rgb_matrix_config();
    let mut position = led_func_data.position.unwrap();
    let mut longest_display_line = 0;
    let cols = config.cols as i32;

    let date = Local::now();
    let time_format = &date.format("%H:%M").to_string();

    canvas.fill(0, 0, 0);

    for (i, departure) in led_func_data
        .bvg_timetable
        .clone()
        .unwrap()
        .iter()
        .enumerate()
    {
        let destination = departure
            .direction
            .clone()
            .unwrap()
            .replace("ß", "ss")
            .replace("ü", "ue")
            .replace("ö", "oe")
            .replace("ä", "ae");

        if destination.len() as i32 > longest_display_line {
            longest_display_line = destination.len() as i32;
        }

        let mut my_pos = position;
        my_pos.y = (i as i32 + 1) * 10;

        let color = get_line_color(
            departure
                .line
                .clone()
                .unwrap()
                .product_name
                .unwrap_or("".to_string())
                .as_str(),
        );

        let text = Text::with_alignment(
            destination.as_str(),
            my_pos,
            MonoTextStyle::new(&FONT_4X6, color.clone()),
            Alignment::Left,
        )
        .draw(&mut *canvas)
        .unwrap();
    }

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::BLACK)
        .stroke_width(3)
        .fill_color(Rgb888::BLACK)
        .build();

    let puffer_width = 18;
    Rectangle::new(
        Point::new(0, 0),
        Size::new(puffer_width, config.rows as u32),
    )
    .into_styled(style)
    .draw(&mut *canvas)
    .expect("to work");

    for (i, departure) in led_func_data
        .bvg_timetable
        .clone()
        .unwrap()
        .iter()
        .enumerate()
    {
        let product_name = departure
            .line
            .clone()
            .unwrap()
            .product_name
            .clone()
            .unwrap();

        let departure_time = DateTime::parse_from_str(
            &departure
                .when
                .clone()
                .unwrap_or("1970-01-01 00:00:00".to_string()),
            "%Y-%m-%dT%H:%M:%S%:z",
        )
        .unwrap()
        .format("%H:%M")
        .to_string();

        let mut time_positon = Point::new(0, 0);
        time_positon.y = (i as i32 + 1) * 10;
        let text = Text::with_alignment(
            departure_time.as_str(),
            time_positon,
            MonoTextStyle::new(&FONT_4X6, Rgb888::CSS_GRAY),
            Alignment::Left,
        )
        .draw(&mut *canvas)
        .unwrap();
    }

    position.x -= 1;

    if position.x == -((cols - puffer_width as i32) + (longest_display_line * 3)) {
        position.x = cols;
    }

    Text::with_alignment(
        time_format.as_str(),
        Point::new(cols / 2, 5),
        MonoTextStyle::new(&FONT_4X6, Rgb888::CSS_GRAY),
        Alignment::Center,
    )
    .draw(&mut *canvas)
    .unwrap();

    return (
        *canvas,
        crate::LedFuncData {
            position: Some(position),
            ..led_func_data
        },
    );
}
