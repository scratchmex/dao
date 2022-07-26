use crate::{ClipboardData, ClipboardError, ClipboardProvider};

#[cfg(target_os = "windows")]
mod win {
    use super::*;
    use clipboard_win::{formats, get_clipboard, is_format_avail, set_clipboard, SystemError};

    #[derive(Debug)]
    pub struct WindowsClipboard;

    impl ClipboardProvider for WindowsClipboard {
        fn new() -> Result<Self, ClipboardError>
        where
            Self: Sized,
        {
            Ok(WindowsClipboard {})
        }

        fn get(&self) -> Result<ClipboardData, ClipboardError> {
            let result = if is_format_avail(formats::CF_BITMAP) {
                ClipboardData::Image(get_clipboard(formats::Bitmap)?.try_into()?)
            } else if is_format_avail(formats::CF_UNICODETEXT) {
                ClipboardData::Text(get_clipboard(formats::Unicode)?)
            } else {
                return Err(ClipboardError::CouldNotRead);
            };

            Ok(result)
        }

        fn set(&self, data: ClipboardData) -> Result<(), ClipboardError> {
            match data {
                ClipboardData::Text(text) => {
                    set_clipboard(formats::Unicode, text).map_err(|e| e.into())
                }
                ClipboardData::Image(_) => todo!(),
            }
        }
    }

    impl From<SystemError> for ClipboardError {
        fn from(err: SystemError) -> Self {
            ClipboardError::SystemError(format!("{}", err))
        }
    }
}

#[cfg(target_os = "windows")]
pub use win::*;
