use std::fs::File;
use std::path::Path;
use crate::level::{TextureDefinition, Textures};
use miette::IntoDiagnostic;
use image::io::Reader as ImageReader;

pub fn update_texture_sizes() -> miette::Result<()> {
    let path = "assets/textures.yaml";

    let f = File::open(path).into_diagnostic()?;
    let mut textures: Textures = serde_yaml::from_reader(f).into_diagnostic()?;

    for entry in glob::glob("assets/**/*.png").into_diagnostic()? {
        dbg!(&entry);
        let path = entry.into_diagnostic()?;
        let path = Path::new(&path);
        let non_asset_path = path.strip_prefix("assets").unwrap();
        let i = ImageReader::open(path).into_diagnostic()?.decode().into_diagnostic()?;
        println!("{:?} {:?} {}", path, i.width(), i.height());

        let mut found = false;
        textures.0.iter_mut().for_each(|mut td| {
            if td.path == non_asset_path.to_str().unwrap() {
                td.size = [i.width(), i.height()];
                found = true;
            }
        });

        if !found {
            textures.0.push(TextureDefinition {
                path: non_asset_path.to_str().unwrap().to_string(),
                size: [i.width(), i.height()],
            });
        }
    }
    let mut f = File::create(path).into_diagnostic()?;
    serde_yaml::to_writer(&mut f, &textures).into_diagnostic()?;

    Ok(())
}
