use arboard::Clipboard as ClipboardContext;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Arboard(#[from] arboard::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub(crate) struct Clipboard {
    context: ClipboardContext,
}

impl Clipboard {
    pub(crate) fn open() -> Result<Self> {
        Ok(Self {
            context: ClipboardContext::new()?,
        })
    }

    pub(crate) fn read_text(&mut self) -> Result<String> {
        self.context.get_text().map_err(|e| e.into())
    }

    pub(crate) fn write_text(&mut self, contents: &&str) -> Result<()> {
        self.context
            .set_text(contents.to_string())
            .map_err(|e| e.into())
    }
}
