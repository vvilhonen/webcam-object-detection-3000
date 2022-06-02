use anyhow::Context;
use image::Rgb;
use image::RgbImage;
use imageproc::drawing::draw_text_mut;

use rusttype::Font;
use rusttype::Scale;

use lazy_static::lazy_static;

lazy_static! {
    static ref FONT: Font<'static> =
        Font::try_from_vec(include_bytes!("../font/AGAALER.TTF").to_vec())
            .expect("failed creating font");
}

pub fn annotate(buf: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let mut image = RgbImage::from_raw(1920, 1080, buf).context("Couldn't load image")?;

    let height = 64.0;
    let scale = Scale {
        x: height * 2.0,
        y: height,
    };

    draw_text_mut(
        &mut image,
        Rgb([255, 0, 0]),
        10,
        10,
        scale,
        &FONT,
        "BLACK GOLD DETECTED",
    );

    let output = image.to_vec();
    Ok(output)
}
