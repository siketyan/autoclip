pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// Plugins must implement this trait.
/// This trait is used to trigger events from the app.
pub trait AutoclipPlugin {
    /// This event will be triggered when the content of the clipboard is updated.
    /// If the plugin does not perform any modification for the content,
    /// this function must return None, instead of `Some(contents.to_string())` .
    fn on_clip(&self, contents: &str) -> Option<String>;
}

/// An internal struct to load plugins from the app.
/// The version of rustc and autoclip-core may be checked on loading.
pub struct PluginDeclaration {
    /// The version of rustc, used to compile the core.
    /// This should be set on build-time.
    pub rustc_version: &'static str,

    /// The version of the core.
    /// This also should be set on build-time.
    pub core_version: &'static str,

    /// The function to register a plugin to the app.
    /// In this function, `PluginRegistrar::register` must be called.
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

/// The app must implement this trait.
/// This trait is used to give plugins a method to register themselves to the app.
pub trait PluginRegistrar {
    fn register(&mut self, name: &str, entrypoint: Box<dyn AutoclipPlugin>);
}

/// An useful macro to export plugins.
/// Using only this macro, each plugin can integrate with the app easily.
#[macro_export]
macro_rules! export_plugin {
    ($name:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register,
        };

        #[allow(improper_ctypes_definitions)]
        extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
            registrar.register($name, Box::new(AutoclipPluginAmazon));
        }
    };
}
