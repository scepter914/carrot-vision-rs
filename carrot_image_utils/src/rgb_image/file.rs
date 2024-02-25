use image::RgbImage;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn save_ppm_file(image: &RgbImage, path: &Path) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(format!("P6\n{} {}\n255\n", image.width(), image.height()).as_bytes())?;
    file.write_all(image)?;
    Ok(())
}
