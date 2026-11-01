//! The system clipboard: text, HTML and PNG images (LOK-06).

#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod board;
#[cfg(any(target_os = "android", target_os = "ios"))]
#[path = "clipboard/phone.rs"]
mod board;

pub use board::{html, read, write};

/// One clipboard payload.
#[derive(Clone, Debug, PartialEq)]
pub enum Clip {
    Text(String),
    /// Rich text or tables, with a plain fallback.
    Html {
        html: String,
        text: String,
    },
    Png(Vec<u8>),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes then reads the real system clipboard.
    #[test]
    fn each_kind_round_trips() {
        if std::env::var_os("ELY_CLIPBOARD_TEST").is_none() {
            eprintln!("SKIP: set ELY_CLIPBOARD_TEST=1; this replaces the clipboard");
            return;
        }
        let text = Clip::Text("plain 中文".into());
        write(&text).expect("text writes");
        assert_eq!(read().expect("text reads"), Some(text));
        assert_eq!(html().expect("no html reads"), None);
        let html = Clip::Html {
            html: "<table><tr><td>a</td></tr></table>".into(),
            text: "a".into(),
        };
        write(&html).expect("html writes");
        let Some(Clip::Html { html: back, text }) = read().expect("html reads") else {
            panic!("html comes back as html");
        };
        assert!(back.contains("<td>a</td>"), "the table survives: {back}");
        assert_eq!(text, "a");
        let mut png = Vec::new();
        image::RgbaImage::from_pixel(3, 2, image::Rgba([0, 0, 255, 255]))
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .expect("the png encodes");
        write(&Clip::Png(png)).expect("png writes");
        let Some(Clip::Png(back)) = read().expect("png reads") else {
            panic!("an image comes back as png");
        };
        let back = image::load_from_memory(&back).expect("decodes").to_rgba8();
        assert_eq!((back.width(), back.height()), (3, 2));
        assert_eq!(back.get_pixel(0, 0).0, [0, 0, 255, 255]);
    }
}
