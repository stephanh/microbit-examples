use embedded_graphics::fonts::font_builder::{FontBuilder, FontBuilderConf};

#[derive(Debug, Copy, Clone)]
pub enum Font24x32Conf {}
impl FontBuilderConf for Font24x32Conf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/font24x32_1bpp.raw");
    const CHAR_HEIGHT: u32 = 32;
    const CHAR_WIDTH: u32 = 24;
    const FONT_IMAGE_WIDTH: u32 = 960;
    fn char_offset(c: char) -> u32 {
        let fallback = '?' as u32 - ' ' as u32;
        if c < ' ' {
            return fallback;
        }
        if c <= '~' {
            return c as u32 - ' ' as u32;
        }
        fallback
    }
}
pub type Font24x32<'a, C> = FontBuilder<'a, C, Font24x32Conf>;
