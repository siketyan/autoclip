use clipboard_win::{seq_num, Clipboard as Handle, Getter, Setter};
use error_code::SystemError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("no access")]
    NoAccess,

    #[error("system error")]
    System(SystemError),
}

type Result<T> = std::result::Result<T, Error>;

pub(crate) struct Clipboard {
    #[allow(dead_code)]
    handle: Handle,
}

impl Clipboard {
    pub(crate) fn open() -> Result<Self> {
        Ok(Clipboard {
            handle: Handle::new().map_err(Error::System)?,
        })
    }

    pub(crate) fn get_sequence_number(&self) -> Result<u32> {
        Ok(seq_num().ok_or_else(|| Error::NoAccess)?.get())
    }

    pub(crate) fn read_text(&self) -> Result<String> {
        let mut output = String::new();

        clipboard_win::formats::Unicode
            .read_clipboard(&mut output)
            .map_err(|e| Error::System(e))?;

        Ok(output)
    }

    pub(crate) fn write_text(&self, contents: &&str) -> Result<()> {
        clipboard_win::formats::Unicode
            .write_clipboard(contents)
            .map_err(|e| Error::System(e))
    }
}
