use iced::window;
use resvg::{tiny_skia, usvg};

use crate::constants::ICON_SVG;

/// Rasterize SVG logo into a window icon
pub fn load_icon() -> Option<window::Icon> {
    let tree = usvg::Tree::from_data(ICON_SVG, &usvg::Options::default()).ok()?;
    let size = 256;
    let mut pixmap = tiny_skia::Pixmap::new(size, size)?;

    let s = tree.size();
    let scale = size as f32 / s.width().max(s.height());
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    // tiny-skia stores premultiplied alpha; winit wants straight alpha
    let rgba: Vec<u8> = pixmap
        .pixels()
        .iter()
        .flat_map(|p| {
            let c = p.demultiply();
            [c.red(), c.green(), c.blue(), c.alpha()]
        })
        .collect();

    window::icon::from_rgba(rgba, size, size).ok()
}
