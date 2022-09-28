use cacao::pasteboard::Pasteboard as CacaoPasteboard;
use objc::runtime::Object;
use objc::*;
use objc_foundation::{INSArray, INSString, NSArray, NSString};
use objc_id::{Id, ShareId};

use crate::platform::GetClipboardTypes;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("the object was unavailable, returned null")]
    ObjectUnavailable,
}

pub(super) struct Pasteboard {
    obj: ShareId<Object>,
}

impl Pasteboard {
    pub(super) fn new() -> Self {
        Self {
            obj: CacaoPasteboard::default().0,
        }
    }
}

impl GetClipboardTypes for Pasteboard {
    type Error = Error;

    fn get_clipboard_types(&self) -> Result<Vec<String>, Self::Error> {
        let types: Id<NSArray<NSString>> = unsafe {
            let types: *mut NSArray<NSString> = msg_send![self.obj, types];
            if types.is_null() {
                return Err(Error::ObjectUnavailable);
            }

            Id::from_ptr(types)
        };

        Ok(types
            .object_enumerator()
            .map(|ty| ty.as_str().to_owned())
            .collect())
    }
}
