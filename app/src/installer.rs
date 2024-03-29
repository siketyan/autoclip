use reqwest::{
    blocking::{Client, Response},
    StatusCode, Url,
};
use serde::Deserialize;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("plugin with the name not found.")]
    PluginNotFound,

    #[error("no variant supports this platform.")]
    SupportedVariantNotFound,

    #[error("registry error")]
    Registry(Box<Response>),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, PartialEq)]
enum Os {
    #[serde(rename = "windows")]
    Windows,

    #[serde(rename = "macos")]
    Macos,

    #[serde(rename = "linux")]
    Linux,
}

impl Os {
    fn is_supported(&self) -> bool {
        match self {
            Os::Windows => cfg!(target_os = "windows"),
            Os::Macos => cfg!(target_os = "macos"),
            Os::Linux => cfg!(target_os = "linux"),
        }
    }
}

#[derive(Deserialize, PartialEq)]
enum Arch {
    #[serde(rename = "x86_32")]
    X8632,

    #[serde(rename = "x86_64")]
    X8664,
}

impl Arch {
    fn is_supported(&self) -> bool {
        match self {
            Arch::X8632 => cfg!(target_arch = "x86_32"),
            Arch::X8664 => cfg!(target_arch = "x86_64"),
        }
    }
}

#[derive(Deserialize)]
struct Variant {
    os: Os,
    arch: Arch,
    url: String,
}

impl Variant {
    fn is_supported(&self) -> bool {
        self.os.is_supported() && self.arch.is_supported()
    }
}

#[derive(Deserialize)]
struct Author {
    name: String,
    email: String,
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} <{}>", self.name, self.email)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Manifest {
    name: String,
    description: String,
    author: Author,
    variants: Vec<Variant>,
}

impl Manifest {
    fn find_supported_variant(&self) -> Option<&Variant> {
        self.variants.iter().find(|&v| v.is_supported())
    }
}

pub(crate) struct Installer {
    client: Client,
    registry: String,
}

impl Installer {
    pub(crate) fn new(registry: Option<&str>) -> Self {
        Self::with_client(Client::new(), registry)
    }

    fn with_client(client: Client, registry: Option<&str>) -> Self {
        Self {
            client,
            registry: registry
                .unwrap_or("https://autoclip-plugins.projects.siketyan.dev/")
                .to_string(),
        }
    }

    fn fetch_manifest(&self, name: &str) -> Result<Manifest> {
        let mut url = Url::from_str(&self.registry)?;

        url.set_path(&format!("/{}.yaml", name));

        let response = self.client.get(url).send()?;
        match response.status() {
            StatusCode::OK => serde_yaml::from_reader(response).map_err(|e| e.into()),
            StatusCode::NOT_FOUND => Err(Error::PluginNotFound),
            _ => Err(Error::Registry(Box::new(response))),
        }
    }

    pub(crate) fn install<P>(&self, name: &str, to: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let manifest = self.fetch_manifest(name)?;
        let variant = manifest
            .find_supported_variant()
            .ok_or(Error::SupportedVariantNotFound)?;

        println!(
            "Installing {}, created by {}, from {}",
            manifest.name, manifest.author, variant.url,
        );

        let response = self.client.get(&variant.url).send()?;
        let filename = format!("{}.{}", manifest.name, plugin_extension());
        let file = File::create(PathBuf::from(to.as_ref()).join(filename))?;

        let mut reader = BufReader::new(response);
        let mut writer = BufWriter::new(file);

        copy(&mut reader, &mut writer)
            .map(|_| ())
            .map_err(Error::Io)
    }
}

fn plugin_extension() -> &'static str {
    #[cfg(windows)]
    return "dll";

    #[cfg(target_os = "macos")]
    return "dylib";

    #[allow(unreachable_code)]
    "so"
}
