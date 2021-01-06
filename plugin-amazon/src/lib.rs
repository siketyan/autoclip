use autoclip_core::{AutoclipPlugin, PluginRegistrar};
use regex::Regex;

autoclip_core::export_plugin!("amazon");

pub struct AutoclipPluginAmazon;

impl AutoclipPlugin for AutoclipPluginAmazon {
    fn on_clip(&self, contents: &str) -> Option<String> {
        let regex = Regex::new(r"(https://www.amazon.co.jp/)(?:.+/)?(dp/[A-Z0-9]+)/?")
            .expect("regex error");

        if !regex.is_match(contents) {
            return None;
        }

        let mut output = String::new();
        let captures = regex.captures(contents).expect("failed to match");

        output.push_str(captures.get(1).expect("get").as_str());
        output.push_str(captures.get(2).expect("get").as_str());

        Some(output)
    }
}
