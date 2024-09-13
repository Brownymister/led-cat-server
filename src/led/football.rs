use crate::api::football::FootballRes;
use crate::api::{football, weatherkit};
use chrono::{DateTime, Local};
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawBE},
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
use image::{DynamicImage, GenericImageView, RgbImage};
use rpi_led_panel::{Canvas, RGBMatrix, RGBMatrixConfig};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

pub async fn show_football(
    led_func_data: crate::LedFuncData,
    mut canvas: Box<Canvas>,
) -> (Canvas, crate::LedFuncData) {
    let config: RGBMatrixConfig = crate::get_rgb_matrix_config();
    let rows = config.rows as i32;
    let cols = config.cols as i32;

    for (i, _match) in led_func_data
        .clone()
        .football
        .unwrap()
        .clone()
        .iter()
        .enumerate()
    {
        log::info!(
            "football {} vs {}",
            _match.team1.shortName,
            _match.team2.shortName
        );

        let mut team1_goals = 0;
        let mut team2_goals = 0;

        for goal in _match.goals.clone() {
            team1_goals = goal.scoreTeam1;
            team2_goals = goal.scoreTeam2;
        }

        Text::with_alignment(
            &(_match.team1.shortName.clone() + " " + &team1_goals.to_string()),
            Point::new(16, ((i + 1) * 10).try_into().unwrap()),
            MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
            Alignment::Center,
        )
        .draw(canvas.as_mut())
        .unwrap();

        Text::with_alignment(
            &(_match.team2.shortName.clone() + " " + &team2_goals.to_string()),
            Point::new(48, ((i + 1) * 10).try_into().unwrap()),
            MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
            Alignment::Center,
        )
        .draw(canvas.as_mut())
        .unwrap();
    }

    return (*canvas, led_func_data);
}
