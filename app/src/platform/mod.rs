#[cfg(target_os = "macos")]
mod macos;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[cfg(target_os = "macos")]
    #[error(transparent)]
    Macos(#[from] macos::Error),

    #[error("this platform is not supported yet.")]
    Unsupported,
}

pub(crate) trait GetClipboardTypes {
    type Error;

    fn get_clipboard_types(&self) -> Result<Vec<String>, Self::Error>;
}

pub(crate) fn get_clipboard_types() -> Result<Vec<String>, Error> {
    #[cfg(target_os = "macos")]
    return macos::Pasteboard::new()
        .get_clipboard_types()
        .map_err(|e| e.into());

    #[allow(unreachable_code)]
    Err(Error::Unsupported)
}
