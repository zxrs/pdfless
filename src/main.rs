use anyhow::Result;
use pdf_thumb::{ImageFormat, Options, PdfDoc};
use ratatui::{DefaultTerminal, Frame, layout::Rect, prelude::Backend};
use ratatui_image::{
    FilterType::Lanczos3,
    Image,
    protocol::{Protocol, sixel::Sixel},
};

struct App {
    image: Protocol,
}

fn main() -> Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> Result<()> {
    let size = terminal.size()?;

    let pdf = PdfDoc::open(r"v.pdf")?;
    let thumb = pdf.thumb_with_options(Options {
        format: ImageFormat::Bmp,
        ..Default::default()
    })?;
    let image = image::load_from_memory_with_format(&thumb, image::ImageFormat::Bmp)?;
    let image = if image.width() > image.height() {
        // TODO: these factor "10" should be calculated...
        image.resize(size.width as u32 * 10, size.width as u32 * 10, Lanczos3)
    } else {
        // TODO: these factor "20" should be calculated...
        image.resize(size.height as u32 * 20, size.height as u32 * 20, Lanczos3)
    };

    let mut app = App {
        image: Protocol::Sixel(Sixel::new(
            image,
            Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: size.height,
            },
            false,
        )?),
    };

    loop {
        terminal.draw(|f| render(f, &mut app))?;
        if crossterm::event::read()?.is_key_press() {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame<'_>, app: &mut App) {
    let image = Image::new(&mut app.image);
    frame.render_widget(image, frame.area());
}
