use image::ImageError;

pub fn get_rbga_from_image(path: &str) -> Result<(u32, u32, Vec<u8>), ImageError> {
    match image::open(path) {
        Ok(img) => {
            let width = img.width();
            let height = img.height();
            let pixel_data = img.to_rgba8().into_raw();
            Ok((width, height, pixel_data))
        },
        Err(e) => Err(e)
    }
}
