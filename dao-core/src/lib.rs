use image::{load_from_memory, DynamicImage, ImageError};
use std::{error::Error, fmt};

mod clipboard;

pub trait ClipboardProvider {
    fn new() -> Result<Self, ClipboardError>
    where
        Self: Sized;

    fn get(&self) -> Result<ClipboardData, ClipboardError>;
    fn set(&self, data: ClipboardData) -> Result<(), ClipboardError>;
}

#[derive(Debug)]
pub enum ClipboardData {
    Text(String),
    Image(ImageData),
}

#[derive(Debug)]
pub enum ClipboardError {
    CouldNotRead,
    SystemError(String),
    ImageError(ImageError),
}

#[derive(Debug)]
pub struct ImageData(pub DynamicImage);

impl TryFrom<Vec<u8>> for ImageData {
    type Error = ImageError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let img = load_from_memory(&value)?;

        Ok(ImageData(img))
    }
}

impl From<ImageData> for DynamicImage {
    fn from(img: ImageData) -> Self {
        img.0
    }
}

impl From<ImageError> for ClipboardError {
    fn from(_: ImageError) -> Self {
        todo!()
    }
}

impl fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg: String = match self {
            ClipboardError::CouldNotRead => "Could not read the clipboard :s".into(),
            ClipboardError::SystemError(err) => format!("SystemError: {}", err),
            ClipboardError::ImageError(err) => format!("ImageError: {}", err),
        };

        f.write_str(&msg)
    }
}

impl Error for ClipboardError {}

#[cfg(target_os = "windows")]
pub use clipboard::WindowsClipboard as Clipboard;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn clipboard_works() {
        let clip = Clipboard::new().expect("could not open clipboard?");
        let text = "does it work?";

        clip.set(ClipboardData::Text(text.into()))
            .expect("could not set clipboard?");
        let content = match clip.get().expect("could not get clipboard?") {
            ClipboardData::Text(text) => text,
            ClipboardData::Image(_) => panic!("I had putten my text here"),
        };

        assert_eq!(text, content)
    }
}
