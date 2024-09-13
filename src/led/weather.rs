use crate::api::weatherkit;
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

pub fn show_weather_forecast(
    led_func_data: crate::LedFuncData,
    mut canvas: Box<Canvas>,
) -> (Canvas, crate::LedFuncData) {
    let config: RGBMatrixConfig = crate::get_rgb_matrix_config();
    let rows = config.rows as i32;
    let cols = config.cols as i32;

    let mut pre_noon = vec![];
    let mut after_noon = vec![];
    let mut forcast_date: String = "".to_string();

    if led_func_data.weather_fercast_day.is_some() {
        let binding = led_func_data.weather_forcast.clone().unwrap().forecastDaily;

        let day_info = binding.days[led_func_data.weather_fercast_day.unwrap() as usize].clone();

        println!("day_info forcast_date: {:?}", day_info.forecastStart);
        forcast_date = DateTime::parse_from_rfc3339(day_info.forecastStart.as_str())
            .unwrap()
            .format("%d.%m")
            .to_string();

        Text::with_alignment(
            &(day_info.temperatureMax.round().to_string() + "°C"),
            Point::new(32, 7),
            MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
            Alignment::Center,
        )
        .draw(canvas.as_mut())
        .unwrap();

        Text::with_alignment(
            &(day_info.temperatureMin.round().to_string() + "°C"),
            Point::new(32, 17),
            MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
            Alignment::Center,
        )
        .draw(canvas.as_mut())
        .unwrap();

        let path = format!("/home/pi/.config/ledcat/icons/{}.png", day_info.conditionCode);
        println!("path: {}", path);
        display_image_by_path(path, canvas.as_mut(), Point::new(22, 28));
    } else {
        let binding = led_func_data
            .weather_forcast
            .clone()
            .unwrap()
            .forecastHourly;

        let len = binding.hours.len();

        for (i, hour) in binding.hours.iter().enumerate() {
            if i < len / 2 {
                pre_noon.push(hour);
            } else {
                after_noon.push(hour);
            }
        }

        Text::with_alignment(
            &(pre_noon
                .iter()
                .max_by(fix_s)
                .map(|a| a.temperature)
                .unwrap_or(0.0)
                .round()
                .to_string()
                + "°C"),
            Point::new(16, 7),
            MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
            Alignment::Center,
        )
        .draw(canvas.as_mut())
        .unwrap();

        Text::with_alignment(
            &(pre_noon
                .iter()
                .min_by(fix_s)
                .map(|a| a.temperature)
                .unwrap_or(0.0)
                .round()
                .to_string()
                + "°C"),
            Point::new(16, 17),
            MonoTextStyle::new(&FONT_5X7, Rgb888::CSS_GRAY),
            Alignment::Center,
        )
        .draw(canvas.as_mut())
        .unwrap();

        Text::with_alignment(
            &(after_noon
                .iter()
                .max_by(fix_s)
                .map(|a| a.temperature)
                .unwrap_or(0.0)
                .round()
                .to_string()
                + "°C"),
            Point::new(48, 7),
            MonoTextStyle::new(&FONT_5X7, Rgb888::WHITE),
            Alignment::Center,
        )
        .draw(canvas.as_mut())
        .unwrap();

        Text::with_alignment(
            &(after_noon
                .iter()
                .min_by(fix_s)
                .map(|a| a.temperature)
                .unwrap_or(0.0)
                .round()
                .to_string()
                + "°C"),
            Point::new(48, 17),
            MonoTextStyle::new(&FONT_5X7, Rgb888::CSS_GRAY),
            Alignment::Center,
        )
        .draw(canvas.as_mut())
        .unwrap();

        let pre_noon_mean_condition = most_frequent(
            pre_noon
                .iter()
                .map(|a| a.conditionCode.clone())
                .collect::<Vec<String>>(),
        );
        let after_noon_mean_condition = most_frequent(
            after_noon
                .iter()
                .map(|a| a.conditionCode.clone())
                .collect::<Vec<String>>(),
        );
        println!(
            "pre_noon_mean_condition: {:?}",
            pre_noon_mean_condition.clone().unwrap_or("".to_string())
        );
        println!(
            "after_noon_mean_condition: {:?}",
            after_noon_mean_condition.clone().unwrap_or("".to_string())
        );

        let path = format!(
            "/home/pi/.config/ledcat/icons/{}.png",
            pre_noon_mean_condition.unwrap_or("".to_string())
        );
        display_image_by_path(path, canvas.as_mut(), Point::new(8, 28));

        let path = format!(
            "/home/pi/.config/ledcat/icons/{}.png",
            after_noon_mean_condition.unwrap_or("".to_string())
        );
        display_image_by_path(path, canvas.as_mut(), Point::new(36, 28));

        forcast_date = DateTime::parse_from_rfc3339(
            led_func_data.weather_forcast.unwrap().forecastDaily.days[0]
                .forecastStart
                .as_str(),
        )
        .unwrap()
        .format("%d.%m")
        .to_string();
    }

    let mut place_str: String = led_func_data
        .osm_info
        .clone()
        .unwrap()
        .address
        .city
        .unwrap_or(
            led_func_data
                .osm_info
                .clone()
                .unwrap()
                .address
                .town
                .unwrap_or(
                    led_func_data
                        .osm_info
                        .clone()
                        .unwrap()
                        .address
                        .leisure
                        .unwrap_or(
                            led_func_data
                                .osm_info
                                .clone()
                                .unwrap()
                                .address
                                .village
                                .unwrap_or("".to_string()),
                        ),
                ),
        );
    if place_str.len() > 8 {
        place_str = place_str.chars().take(8).collect::<String>() + "..";
    }

    Text::with_alignment(
        &forcast_date,
        Point::new(1, 60),
        MonoTextStyle::new(&FONT_4X6, Rgb888::CSS_GRAY),
        Alignment::Left,
    )
    .draw(canvas.as_mut())
    .unwrap();

    Text::with_alignment(
        &place_str,
        Point::new(63, 60),
        MonoTextStyle::new(&FONT_4X6, Rgb888::CSS_GRAY),
        Alignment::Right,
    )
    .draw(canvas.as_mut())
    .unwrap();

    return (
        *canvas,
        crate::LedFuncData {
            ..Default::default()
        },
    );
}

fn fix_s(
    a: &&&weatherkit::WeatherHourlyForcastHours,
    b: &&&weatherkit::WeatherHourlyForcastHours,
) -> std::cmp::Ordering {
    return match a.temperature.partial_cmp(&b.temperature) {
        Some(std::cmp::Ordering::Greater) => std::cmp::Ordering::Greater,
        Some(std::cmp::Ordering::Less) => std::cmp::Ordering::Less,
        Some(std::cmp::Ordering::Equal) => std::cmp::Ordering::Equal,
        None => std::cmp::Ordering::Equal, // Treat NaN and infinity as equal
    };
}

fn most_frequent(strings: Vec<String>) -> Option<String> {
    let mut counts = HashMap::new();
    for string in strings {
        let count = counts.entry(string).or_insert(0);
        *count += 1;
    }

    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(string, _)| string)
}

fn load_file(path: &str) -> std::io::Result<Vec<u8>> {
    std::fs::read(path)
}

pub fn display_image_by_path(path: String, canvas: &mut Canvas, pos: Point) {
    let img = image::open(path).expect("Failed to open image");

    // Convert the dynamic image to an RGB image buffer
    let rgb_img = match img {
        DynamicImage::ImageRgb8(buffer) => buffer,
        _ => img.to_rgb8(),
    };

    let image_data = rgb_img.into_raw();

    let image_data = ImageRawBE::<Rgb888>::new(&image_data, 20 as u32);
    let image = Image::new(&image_data, pos);

    image.draw(canvas).unwrap();
}
