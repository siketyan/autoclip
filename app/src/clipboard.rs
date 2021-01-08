use clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Std(Box<dyn std::error::Error>),
}

type Result<T> = std::result::Result<T, Error>;

pub(crate) struct Clipboard {
    context: ClipboardContext,
}

impl Clipboard {
    pub(crate) fn open() -> Result<Self> {
        Ok(Self {
            context: ClipboardProvider::new().map_err(Error::Std)?,
        })
    }

    pub(crate) fn read_text(&mut self) -> Result<String> {
        self.context.get_contents().map_err(Error::Std)
    }

    pub(crate) fn write_text(&mut self, contents: &&str) -> Result<()> {
        self.context
            .set_contents(contents.to_string())
            .map_err(Error::Std)
    }
}
