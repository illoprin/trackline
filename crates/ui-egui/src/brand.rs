//! The Trackline app mark, decoded once and kept as a texture and drawn in the About window and
//! on the Home (Import) screen.
//!
//! The artwork is the fork's own logo (`plan/icon.svg`, derived to
//! `assets/app-icon/hicolor/256x256/apps/ru.illoprin.trackline.png` by `packaging/icons.sh`). It
//! is not an ArtCraft mark; the ArtCraft name and logos are trademarks of the ArtCraft Team and
//! are not used by this fork (see `docs/brand/LICENSE-brand.txt`).

const LOGO: &[u8] = include_bytes!("../../../assets/app-icon/hicolor/256x256/apps/ru.illoprin.trackline.png");

/// The app mark as a texture, decoded once and cached. Its pixel size is a square.
pub fn logo_texture(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let id = egui::Id::unique("trackline-logo");
    if let Some(t) = ctx.data(|d| d.get_temp::<egui::TextureHandle>(id)) {
        return Some(t);
    }
    let img = image::load_from_memory_with_format(LOGO, image::ImageFormat::Png).ok()?;
    // Large enough to stay sharp at 2x for the sizes we draw (≤ 320 pt wide).
    let img = img.resize(512, 512, image::imageops::FilterType::Lanczos3).to_rgba8();
    let color = egui::ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.as_raw());
    let tex = ctx.load_texture("trackline-logo", color, egui::TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
    Some(tex)
}

/// Draw the mark `height` points tall with its left edge at `left_center` (vertically centred).
/// Returns the drawn rect.
pub fn paint_logo(ui: &egui::Ui, left_center: egui::Pos2, height: f32) -> Option<egui::Rect> {
    let tex = logo_texture(ui.ctx())?;
    let [w, h] = tex.size();
    let width = height * w as f32 / h.max(1) as f32;
    let r = egui::Rect::from_min_size(egui::pos2(left_center.x, left_center.y - height / 2.0), egui::vec2(width, height));
    ui.painter().image(tex.id(), r, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
    Some(r)
}
