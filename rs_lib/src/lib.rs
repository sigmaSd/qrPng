use wasm_bindgen::prelude::*;

use image::{ImageEncoder, Luma};
use qrcode::QrCode;

#[wasm_bindgen]
/// Generates a PNG image of a QR code from the given input data.
pub fn qr_png(data: &[u8]) -> std::result::Result<Vec<u8>, String> {
    // Encode some data into bits.
    let code = QrCode::new(&data).map_err(|e| e.to_string())?;

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    let width = image.width();
    let height = image.height();

    let mut png_buffer = Vec::new();

    image::codecs::png::PngEncoder::new(&mut png_buffer)
        .write_image(
            &image.into_vec(),
            width,
            height,
            image::ExtendedColorType::L8,
        )
        .map_err(|e| e.to_string())?;

    // Return the PNG bytes
    Ok(png_buffer)
}
