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

pub async fn led_run_text(
    led_func_data: crate::LedFuncData,
    mut canvas: Box<Canvas>,
) -> (Canvas, crate::LedFuncData) {
    let now = std::time::Instant::now();
    let config: RGBMatrixConfig = crate::get_rgb_matrix_config();
    let rows = config.rows as i32;
    let cols = config.cols as i32;
    let mut position = led_func_data.position.unwrap().clone();
    let binding = led_func_data.info_message.clone().unwrap();
    let mut color = Rgb888::CYAN;

    if led_func_data.color.is_some() {
        let (r, g, b) = crate::util::hex_to_rgb(led_func_data.color.as_ref().unwrap()).unwrap();
        color = Rgb888::new(r, g, b);
    }

    position.x -= 1;

    if position.x == -(cols + (binding.len() * 5) as i32) {
        position.x = cols;
    }

    log::info!("led_run_text after init {}ms", now.elapsed().as_millis());

    let text = Text::with_alignment(
        binding.as_str(),
        position,
        MonoTextStyle::new(&FONT_10X20, color),
        Alignment::Left,
    );
    log::info!(
        "led_run_text after text init {}ms",
        now.elapsed().as_millis()
    );
    log::info!("updated position: {:?}", position);

    canvas.fill(0, 0, 0);
    text.draw(canvas.as_mut()).unwrap();
    log::info!("led_run_text after text draw {}", now.elapsed().as_millis());
    return (
        *canvas,
        crate::LedFuncData {
            position: Some(position),
            ..led_func_data
        },
    );
}
