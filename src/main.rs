use anyhow::Result;
use icy_sixel::SixelImage;
use pdf_thumb::{ImageFormat, Options, PdfDoc};

fn main() -> Result<()> {
    let pdf = PdfDoc::open(r"test.pdf")?;
    let thumb = pdf.thumb_with_options(Options {
        width: 800,
        format: ImageFormat::Bmp,
        ..Default::default()
    })?;
    let image = image::load_from_memory_with_format(&thumb, image::ImageFormat::Bmp)?;
    let width = image.width() as usize;
    let height = image.height() as usize;
    let rgba = image.to_rgba8().to_vec();
    let sixel_image = SixelImage::try_from_rgba(rgba, width, height)?;
    let sixel = sixel_image.encode()?;
    print!("{sixel}");
    Ok(())
}
