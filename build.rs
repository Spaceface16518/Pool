use std::convert::TryInto;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use image::{ImageError, Pixel, RgbaImage};
use image::Rgba;
use sheep::{AmethystFormat, InputSprite, SimplePacker};

fn main() -> Result<(), Box<dyn Error>> {
    let particle = particle_sprite()?;

    let results =
        sheep::pack::<SimplePacker>(vec![particle], Rgba::<u8>::CHANNEL_COUNT.try_into()?, ());
    assert_eq!(results.len(), 1);
    let sprite_sheet = results.into_iter().next().unwrap();

    let meta = sheep::encode::<AmethystFormat>(&sprite_sheet, ());

    let outbuf = RgbaImage::from_vec(
        sprite_sheet.dimensions.0,
        sprite_sheet.dimensions.1,
        sprite_sheet.bytes,
    )
        .ok_or_else(|| "Could not construct sprite sheet from bytes".to_owned())?;

    outbuf.save("resources/sprite_sheet.png")?;

    let mut meta_file = File::create("resources/sprite_sheet.ron")?;
    let meta_str = ron::ser::to_string(&meta)?;

    meta_file.write_all(meta_str.as_bytes())?;

    Ok(())
}

fn particle_sprite() -> Result<InputSprite, ImageError> {
    let img = image::open("resources/sprites/particle.png")?;
    let img_owned;
    let img = {
        if let Some(img) = img.as_rgba8() {
            img
        } else {
            img_owned = img.to_rgba();
            &img_owned
        }
    };

    let dimensions = img.dimensions();
    let bytes = img
        .pixels()
        .flat_map(|pixel| pixel.0.iter().copied())
        .collect();

    Ok(InputSprite { dimensions, bytes })
}
