use autoclip_core::{AutoclipPlugin, PluginDeclaration};
use libloading::Library;

use std::ffi::OsStr;
use std::rc::Rc;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("libloading error: {0}")]
    LibLoading(#[from] libloading::Error),

    #[error("version mismatch")]
    Version,
}

type Result<T> = std::result::Result<T, Error>;

fn check_version(declaration: &PluginDeclaration) -> bool {
    declaration.rustc_version != autoclip_core::RUSTC_VERSION
        || declaration.core_version != autoclip_core::CORE_VERSION
}

pub(crate) struct PluginRef {
    pub(crate) name: String,
    pub(crate) plugin: Box<dyn AutoclipPlugin>,

    #[allow(dead_code)]
    pub(crate) library: Rc<Library>,
}

pub(crate) struct PluginCollection {
    plugins: Vec<PluginRef>,
}

impl PluginCollection {
    pub(crate) fn new() -> PluginCollection {
        PluginCollection {
            plugins: Vec::new(),
        }
    }

    pub(crate) unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> Result<&PluginRef> {
        let library = Rc::new(Library::new(library_path).expect("lib load"));

        let declaration = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration")
            .map_err(Error::LibLoading)?
            .read();

        if check_version(&declaration) {
            return Err(Error::Version);
        }

        let mut registrar = PluginRegistrar::new(Rc::clone(&library));

        registrar.register_by(declaration.register);
        self.plugins.push(registrar.plugin_ref.unwrap());

        Ok(self.plugins.last().unwrap())
    }

    pub(crate) fn dispatch_on_clip(&self, contents: &str) -> Option<String> {
        for plugin_ref in &self.plugins {
            let result = plugin_ref.plugin.on_clip(contents);

            if result.is_some() {
                return result;
            }
        }

        None
    }
}

struct PluginRegistrar {
    plugin_ref: Option<PluginRef>,
    library: Rc<Library>,
}

impl PluginRegistrar {
    fn new(library: Rc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            plugin_ref: None,
            library,
        }
    }

    unsafe fn register_by(
        &mut self,
        function: unsafe extern "C" fn(&mut dyn autoclip_core::PluginRegistrar) -> (),
    ) {
        function(self);
    }
}

impl autoclip_core::PluginRegistrar for PluginRegistrar {
    fn register(&mut self, name: &str, plugin: Box<dyn AutoclipPlugin>) {
        self.plugin_ref = Some(PluginRef {
            name: name.to_string(),
            plugin,
            library: Rc::clone(&self.library),
        });
    }
}
