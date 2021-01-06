pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait AutoclipPlugin {
    fn on_clip(&self, contents: &str) -> Option<String>;
}

pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

pub trait PluginRegistrar {
    fn register(&mut self, name: &str, entrypoint: Box<dyn AutoclipPlugin>);
}

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
