use std::borrow::Cow;
use std::sync::OnceLock;
use iced::{font, Pixels};

pub static REGULAR: iced::Font = iced::Font::with_name("Inter Variable");
pub static REGULAR_ITALICS: iced::Font = iced::Font::with_name("Inter Variable Italic");
pub static MONO: iced::Font = iced::Font::with_name("Iosevka");
pub static MONO_BOLD: iced::Font = iced::Font::with_name("Iosevka Bold");
pub const ICON: iced::Font = iced::Font::with_name("Font Awesome 6 Free");

#[derive(Debug, Clone)]
pub struct Font {
    bold: bool,
    italics: bool,
    inner: OnceLock<iced::Font>,
}

pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!("../fonts/Inter.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/InterVariable.ttf")
            .as_slice()
            .into(),
        include_bytes!("../fonts/InterVariable-Italic.ttf")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Iosevka-Regular.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Iosevka-Bold.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Iosevka-Heavy.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Iosevka-ExtraBold.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Iosevka-ExtraLight.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Iosevka-Light.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Iosevka-Medium.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Iosevka-SemiBold.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Iosevka-Thin.ttc")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Font Awesome 6 Free-Regular-400.otf")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Font Awesome 6 Free-Solid-900.otf")
            .as_slice()
            .into(),
        include_bytes!("../fonts/Font Awesome 6 Brands-Regular-400.otf")
            .as_slice()
            .into()
    ]
}

pub fn width_from_chars(len: usize) -> f32 {
    use iced::advanced::graphics::text::Paragraph;
    use iced::advanced::text::{self, Paragraph as _, Text};
    use iced::{alignment, Size};

    Paragraph::with_text(Text {
        content: &" ".repeat(len),
        bounds: Size::INFINITY,
        size: Pixels::from(8.0),
        line_height: Default::default(),
        font: MONO.clone().into(),
        horizontal_alignment: alignment::Horizontal::Right,
        vertical_alignment: alignment::Vertical::Top,
        shaping: text::Shaping::Basic,
        wrapping: Default::default(),
    })
        .min_bounds()
        .expand(Size::new(1.0, 0.0))
        .width
}