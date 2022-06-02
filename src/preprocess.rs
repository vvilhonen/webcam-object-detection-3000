pub fn resize(buf: Vec<u8>) -> Vec<u8> {
    let img = image::RgbImage::from_raw(1920, 1080, buf).unwrap();
    let resized = image::imageops::resize(&img, 320, 180, image::imageops::FilterType::Nearest);
    resized.to_vec()
}
