use std::cell::RefCell;

use anyhow::Result;
use pdf_thumb::{ImageFormat, Options, PdfDoc};
use ratatui::{DefaultTerminal, Frame, layout::Rect};
use ratatui_image::{
    Image,
    protocol::{Protocol, sixel::Sixel},
};

thread_local! {
    static SIXEL: RefCell<Option<String>> = RefCell::new(None);
}

struct App {
    image: Protocol,
}

fn main() -> Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> Result<()> {
    let pdf = PdfDoc::open(r"test.pdf")?;
    let thumb = pdf.thumb_with_options(Options {
        width: 800,
        format: ImageFormat::Bmp,
        ..Default::default()
    })?;
    let image = image::load_from_memory_with_format(&thumb, image::ImageFormat::Bmp)?;
    let width = image.width() as u16;
    let height = image.height() as u16;

    // print!("{sixel}");
    let mut app = App {
        image: Protocol::Sixel(Sixel::new(
            image,
            Rect {
                x: 0,
                y: 0,
                width: 130,
                height: 50,
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
