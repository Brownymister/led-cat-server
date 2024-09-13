use embedded_graphics::{
    mono_font::{
        iso_8859_1::{FONT_10X20, FONT_4X6, FONT_5X7, FONT_5X8, FONT_6X9, FONT_9X15},
        MonoTextStyle,
    },
    pixelcolor::{BinaryColor, Rgb888},
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
    Drawable,
};
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use rpi_led_panel::{Canvas, RGBMatrix, RGBMatrixConfig};

pub fn display_error(
    led_func_data: crate::LedFuncData,
    mut canvas: Box<Canvas>,
) -> (Canvas, crate::LedFuncData) {
    let config: RGBMatrixConfig = crate::get_rgb_matrix_config();
    let rows = config.rows as i32;
    let cols = config.cols as i32;
    let mut color = Rgb888::CYAN;

    // let text = Text::with_alignment(
    //     led_func_data.clone().display_error_text.unwrap().as_str(),
    //     Point::new(32, 32),
    //     Alignment::Center,
    // )
    // .draw(canvas.as_mut())
    // .unwrap();
    //
    let text = "Hello, World!\n\
    A paragraph is a number of lines that end with a manual newline. Paragraph spacing is the \
    number of pixels between two paragraphs.\n\
    Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when \
    an unknown printer took a galley of type and scrambled it to make a type specimen book.";

    // Specify the styling options:
    // * Use the 6x10 MonoFont from embedded-graphics.
    // * Draw the text fully justified.
    // * Use `FitToText` height mode to stretch the text box to the exact height of the text.
    // * Draw the text with `BinaryColor::On`, which will be displayed as light blue.
    //     MonoTextStyle::new(&FONT_5X7, Rgb888::CSS_GRAY),
    let character_style = MonoTextStyle::new(&FONT_5X7, Rgb888::CSS_GRAY);
    let textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::FitToText)
        .alignment(HorizontalAlignment::Justified)
        .paragraph_spacing(6)
        .build();

    // Specify the bounding box. Note the 0px height. The `FitToText` height mode will
    // measure and adjust the height of the text box in `into_styled()`.
    let bounds = Rectangle::new(Point::new(0, 21), Size::new(64, 0));

    // Create the text box and apply styling options.
    let display_error_text = &led_func_data.clone().display_error_text.unwrap();
    let text_box = TextBox::with_textbox_style(
        display_error_text.as_str(),
        bounds,
        character_style,
        textbox_style,
    );

    text_box.draw(canvas.as_mut()).unwrap();

    let path = "/home/pi/.config/ledcat/icons/Error.png".to_string();
    println!("path: {}", path);
    crate::led::weather::display_image_by_path(path, &mut canvas, Point::new(22, 0));

    return (*canvas, crate::LedFuncData { ..led_func_data });
}
