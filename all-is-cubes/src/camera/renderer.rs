use embedded_graphics::mono_font::iso_8859_1::FONT_7X13_BOLD;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::{PixelColor, Point};
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;





/// Provides the standard text style and positioning to draw the “debug info text”
/// (as in [`HeadlessRenderer::draw()`]'s parameter).
///
/// Note: The conventional color is white with a black drop-shadow, but the exact color
/// format and means by which the shadow is accomplished depends on the specific renderer,
/// so this function makes no assumption about color.
#[doc(hidden)]
pub(crate) fn info_text_drawable<C: PixelColor + 'static>(
    text: &str,
    color_value: C,
) -> impl Drawable<Color = C> + '_ {
    Text::with_baseline(
        text,
        Point::new(5, 5),
        MonoTextStyle::new(&FONT_7X13_BOLD, color_value),
        Baseline::Top,
    )
}
