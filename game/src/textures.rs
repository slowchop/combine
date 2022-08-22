use bevy::prelude::Vec2;
use image::io::Reader as ImageReader;
use miette::IntoDiagnostic;
use shared::game::level::Textures;
use shared::game::managed_game::TextureDefinition;
use std::fs::File;
use std::path::Path;

pub fn update_texture_sizes() -> miette::Result<()> {
    let path = "assets/textures.yaml";

    let f = File::open(path).into_diagnostic()?;
    let mut textures: Textures = serde_yaml::from_reader(f).into_diagnostic()?;

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

        let mut found = false;
        textures.0.iter_mut().for_each(|mut td| {
            if td.path == non_asset_path.to_str().unwrap() {
                td.size = Vec2::new(i.width() as f32, i.height() as f32);
                found = true;
            }
        });

        if !found {
            textures.0.push(TextureDefinition {
                path: non_asset_path.to_str().unwrap().to_string(),
                size: Vec2::new(i.width() as f32, i.height() as f32),
            });
        }
    }
    let mut f = File::create(path).into_diagnostic()?;
    serde_yaml::to_writer(&mut f, &textures).into_diagnostic()?;

    Ok(())
}
