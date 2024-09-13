use core::panic;

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

use crate::pong::BallPos;

use super::led;

pub async fn show_pong_led(
    led_func_data: crate::LedFuncData,
    mut canvas: Box<Canvas>,
) -> (Canvas, crate::LedFuncData) {
    let now = std::time::Instant::now();
    let config: RGBMatrixConfig = crate::get_rgb_matrix_config();
    let rows = config.rows as i32;
    let cols = config.cols as i32;

    let mutex = led_func_data.pong_game.clone().unwrap();
    let mut guard = mutex.lock().await;

    guard.move_ball();

    Text::with_alignment(
        &guard.counter.to_string(),
        Point::new(32, 16),
        MonoTextStyle::new(&FONT_6X9, Rgb888::CSS_LIGHT_GRAY),
        Alignment::Center,
    )
    .draw(canvas.as_mut())
    .unwrap();

    let player_pos = guard.player_y_pos.clone();
    Line::new(
        Point::new(guard.player_x_pos.clone(), (player_pos - 1) as i32),
        Point::new(guard.player_x_pos, (player_pos + 1) as i32),
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
    .draw(&mut *canvas)
    .unwrap();

    let ball_pos = guard.ball_pos.clone();
    Line::new(
        Point::new(ball_pos.x, ball_pos.y),
        Point::new(ball_pos.x, ball_pos.y),
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb888::BLUE, 1))
    .draw(&mut *canvas)
    .unwrap();

    return (*canvas, crate::LedFuncData { ..led_func_data });
    // return Box::pin((*canvas, crate::LedFuncData { ..led_func_data }));
}
