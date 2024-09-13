use crate::{api::weatherkit, http_endpoints::DashboardWidgets};
use actix_web::HttpResponse;
use chrono::{DateTime, Local};
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawBE},
    mono_font::{
        iso_8859_1::{FONT_10X20, FONT_4X6, FONT_5X7, FONT_5X8, FONT_6X9, FONT_7X13, FONT_9X15},
        MonoTextStyle,
    },
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
    Drawable,
};
use image::{DynamicImage, GenericImageView, RgbImage};
use rand::Rng;
use rpi_led_panel::{Canvas, RGBMatrix, RGBMatrixConfig};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

pub async fn show_dashboard(
    led_func_data: crate::LedFuncData,
    mut canvas: Box<Canvas>,
) -> (Canvas, crate::LedFuncData) {
    let config: RGBMatrixConfig = crate::get_rgb_matrix_config();
    let rows = config.rows as i32;
    let cols = config.cols as i32;

    let date_time = Local::now();

    let time = date_time.format("%H:%M").to_string();
    let date = date_time.format("%a,%d %b").to_string();

    Text::with_alignment(
        &date,
        Point::new(cols / 2, 10),
        MonoTextStyle::new(&FONT_6X9, Rgb888::CSS_LIGHT_GRAY),
        Alignment::Center,
    )
    .draw(canvas.as_mut())
    .unwrap();

    Text::with_alignment(
        &time,
        Point::new(cols / 2, 25),
        MonoTextStyle::new(&FONT_7X13, Rgb888::CSS_LIGHT_GRAY),
        Alignment::Center,
    )
    .draw(canvas.as_mut())
    .unwrap();

    let widget = led_func_data.dashboard_widget.clone().unwrap();
    let mut position = led_func_data.position.unwrap().clone();

    match widget {
        DashboardWidgets::AutobahnInfo(i) => dashboard_autobahn(
            &mut canvas,
            led_func_data.autobahn_warn.clone().unwrap(),
            &mut position,
        ),
        DashboardWidgets::WeatherInfo(i) => {
            dashboard_weather(&mut canvas, led_func_data.weather_forcast.clone().unwrap())
        }
        _ => Ok(()),
    }
    .unwrap();

    return (
        *canvas,
        crate::LedFuncData {
            position: Some(position),
            ..led_func_data
        },
    );
}

fn dashboard_weather(
    canvas: &mut Canvas,
    weather_forcast: crate::api::weatherkit::WeatherForcast,
) -> Result<(), std::io::Error> {
    let binding = weather_forcast.clone().forecastDaily;

    let day_info = binding.days[0].clone();

    println!("day_info forcast_date: {:?}", day_info.forecastStart);

    Text::with_alignment(
        &(day_info.temperatureMax.round().to_string() + "°C"),
        Point::new(16, 32 + 8),
        MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
        Alignment::Center,
    )
    .draw(canvas)
    .unwrap();

    Text::with_alignment(
        &(day_info.temperatureMin.round().to_string() + "°C"),
        Point::new(16, 32 + 8 + 10),
        MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
        Alignment::Center,
    )
    .draw(canvas)
    .unwrap();

    let path = format!("/home/pi/.config/ledcat/icons/{}.png", day_info.conditionCode);
    println!("path: {}", path);
    crate::led::weather::display_image_by_path(path, canvas, Point::new(32, 32));

    return Ok(());
}

fn dashboard_autobahn(
    canvas: &mut Canvas,
    autobahn_warn: Vec<crate::api::autobahn::AutobahnWarning>,
    position: &mut Point,
) -> Result<(), std::io::Error> {
    let binding = autobahn_warn.clone();

    if binding.is_empty() {
        Text::with_alignment(
            "Keine Staus",
            Point::new(32, 32 + 16),
            MonoTextStyle::new(&FONT_4X6, Rgb888::CYAN),
            Alignment::Center,
        )
        .draw(canvas)
        .unwrap();
        return Ok(());
    }

    let first_warning = binding[0].clone();

    println!("day_info forcast_date: {:?}", first_warning);

    position.x -= 1;

    let cols = crate::get_rgb_matrix_config().cols as i32;

    if position.x == -(cols + (first_warning.title.len() * 4) as i32) {
        position.x = cols; //+ (first_warning.title.len() * 5) as i32;
    }

    Text::with_alignment(
        &(first_warning.title),
        *position,
        MonoTextStyle::new(&FONT_4X6, Rgb888::CYAN),
        Alignment::Left,
    )
    .draw(canvas)
    .unwrap();

    Text::with_alignment(
        &(crate::api::autobahn::match_trafic_type(first_warning.abnormalTrafficType.unwrap())),
        Point::new(32, 32 + 8 + 8),
        MonoTextStyle::new(&FONT_4X6, Rgb888::CYAN),
        Alignment::Center,
    )
    .draw(canvas)
    .unwrap();

    let mut avg_speed = first_warning.averageSpeed.clone().unwrap()[0..2].to_owned();
    if first_warning
        .averageSpeed
        .clone()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()[2]
        != '.'
    {
        avg_speed = first_warning.averageSpeed.clone().unwrap()[0..3].to_owned();
    }

    Text::with_alignment(
        &("ø".to_owned() + &avg_speed + "km/h"),
        Point::new(32, 32 + 8 + 8 + 8),
        MonoTextStyle::new(&FONT_4X6, Rgb888::CYAN),
        Alignment::Center,
    )
    .draw(canvas)
    .unwrap();

    return Ok(());
}
