use anyhow::{Context, Result};
use image::DynamicImage;
use minifb::{Key, Window, WindowOptions};
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;

pub fn scan_qr_code() -> Result<Option<String>> {
    let index = CameraIndex::Index(0);
    // Highest resolution available
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);

    let mut camera = Camera::new(index, requested).context("Failed to init camera")?;
    camera
        .open_stream()
        .context("Failed to open camera stream")?;

    let res = camera.resolution();
    // width and height can be large, we might want to scale it down, but let's just show it.
    let mut window = Window::new(
        "QR Scanner (Press ESC to exit)",
        res.width() as usize,
        res.height() as usize,
        WindowOptions::default(),
    )
    .context("Failed to create minfb window")?;

    let mut scanned_text: Option<String> = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame = camera.frame().context("Failed to capture frame")?;
        let decoded = frame
            .decode_image::<RgbFormat>()
            .context("Failed to decode frame")?;

        // rqrr analysis
        let dyn_img = DynamicImage::ImageRgb8(decoded.clone());
        let luma_img = dyn_img.into_luma8();

        let mut img = rqrr::PreparedImage::prepare_from_greyscale(
            luma_img.width() as usize,
            luma_img.height() as usize,
            |x, y| luma_img.get_pixel(x as u32, y as u32).0[0],
        );
        let grids = img.detect_grids();
        if let Some(grid) = grids.first() {
            if let Ok((_, content)) = grid.decode() {
                scanned_text = Some(content);
                break;
            }
        }

        // Draw onto window
        let mut buffer: Vec<u32> = vec![0; (res.width() * res.height()) as usize];
        for (i, pixel) in decoded.pixels().enumerate() {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            buffer[i] = (r << 16) | (g << 8) | b;
        }

        window
            .update_with_buffer(&buffer, res.width() as usize, res.height() as usize)
            .context("Window update failed")?;
    }

    camera.stop_stream()?;
    Ok(scanned_text)
}
