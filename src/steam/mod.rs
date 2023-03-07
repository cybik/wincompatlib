use std::collections::HashMap;
use std::ffi::{OsString, OsStr};
use std::os::unix::prelude::OsStringExt;
use std::path::PathBuf;
use std::io::{Error, ErrorKind, Result};
use std::process::{Command, Stdio, Output};
use std::str::FromStr;

mod with_ext;
mod boot_ext;
mod run_ext;

pub use with_ext::SteamWithExt;
pub use boot_ext::SteamBootExt;
pub use run_ext::SteamRunExt;

pub use derive_builder::Builder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Steam {
    binary: PathBuf,

    /// Specifies `WINEPREFIX` variable
    pub prefix: Option<PathBuf>,

    /// Path to wineboot binary
    pub wineboot: Option<PathBuf>
}

impl Default for Steam {
    fn default() -> Self {
        Self::from_binary("proton")
    }
}

impl Steam {
    pub fn new<T: Into<PathBuf>>(binary: T, prefix: Option<T>, wineboot: Option<T>) -> Self {
        Steam {
            binary: binary.into(),
            prefix: prefix.map(|value| value.into()),
            wineboot: wineboot.map(|value| value.into())
        }
    }

    pub fn from_binary<T: Into<PathBuf>>(binary: T) -> Self {
        Self::new(binary, None, None)
    }

    /// Try to get version of provided wine binary. Runs command: `wine --version`
    /// 
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// match Steam::default().version() {
    ///     Ok(version) => println!("Steam version: {:?}", version),
    ///     Err(err) => eprintln!("Steam is not available: {}", err)
    /// }
    /// ```
    pub fn version(&self) -> Result<OsString> {
        let output = Command::new(&self.binary)
           .arg("--version")
           .stdout(Stdio::piped())
           .stderr(Stdio::null())
           .output()?;
        /// TODO: load file from the steam install dir and give that value.

        Ok(OsString::from("lol"))
    }

    /// Get wine binary path
    pub fn binary(&self) -> PathBuf {
        self.binary.clone()
    }

    fn get_inner_binary(&self, binary: &str) -> PathBuf {
        if let Some(parent) = self.binary.parent() {
            let binary_path = parent.join(binary);

            if binary_path.exists() {
                return binary_path;
            }
        }

        PathBuf::from(binary)
    }

    /// Get path to wineboot binary, or "wineboot" if not specified
    /// 
    /// If wine binary is specified (so not system), then function will try to find wineboot binary inside of this wine's folder
    /// 
    /// ```no_run
    /// use wincompatlib::prelude::*;
    /// 
    /// use std::path::PathBuf;
    /// 
    /// assert_eq!(Steam::default().wineboot(), PathBuf::from("wineboot"));
    /// assert_eq!(Steam::from_binary("/wine_build/wine").wineboot(), PathBuf::from("/wine_build/wineboot"));
    /// assert_eq!(Steam::from_binary("/wine_build_without_wineboot/wine").wineboot(), PathBuf::from("wineboot"));
    /// ```
    pub fn wineboot(&self) -> PathBuf {
        self.wineboot.clone().unwrap_or_else(|| self.get_inner_binary("wineboot"))
    }

    /// Get environment variables map from current struct's values
    /// 
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// use std::process::Command;
    /// 
    /// let wine = Steam::default().with_arch(SteamArch::Win64);
    /// 
    /// Command::new(wine.binary())
    ///     .envs(wine.get_envs())
    ///     .spawn();
    /// ```
    pub fn get_envs(&self) -> HashMap<&str, OsString> {
        let mut env = HashMap::new();

        if let Some(prefix) = &self.prefix {
            env.insert("WINEPREFIX", prefix.as_os_str().to_os_string());
        }

        env
    }
}
