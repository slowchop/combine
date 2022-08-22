use bevy::prelude::Vec2;
use image::io::Reader as ImageReader;
use miette::IntoDiagnostic;
use shared::game::defs::{Defs, TextureDefinition};
use std::path::Path;

pub fn update_texture_sizes() -> miette::Result<()> {
    let mut defs = Defs::load();

    for entry in glob::glob("assets/**/*.png").into_diagnostic()? {
        dbg!(&entry);
        let path = entry.into_diagnostic()?;
        let path = Path::new(&path);
        let non_asset_path = path.strip_prefix("../assets").unwrap();
        let i = ImageReader::open(path)
            .into_diagnostic()?
            .decode()
            .into_diagnostic()?;
        println!("{:?} {:?} {}", path, i.width(), i.height());

        let size = Vec2::new(i.width() as f32, i.height() as f32);

        defs.textures
            .entry(non_asset_path.to_str().unwrap().to_string())
            .and_modify(|texture| {
                texture.size = Vec2::new(i.width() as f32, i.height() as f32);
                found = true;
            })
            .or_insert(TextureDefinition { size });
    }

    defs.save();

    Ok(())
}
