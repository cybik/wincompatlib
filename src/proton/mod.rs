use std::collections::HashMap;
use std::ffi::{OsString, OsStr};
use std::os::unix::prelude::OsStringExt;
use std::path::PathBuf;
use std::io::{Error, ErrorKind, Result};
use std::process::{Command, Stdio, Output};

mod with_ext;
mod boot_ext;
mod run_ext;

pub use with_ext::ProtonWithExt;
pub use boot_ext::ProtonBootExt;
pub use run_ext::ProtonRunExt;

pub use derive_builder::Builder;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProtonArch {
    Win32,
    Win64
}

impl ProtonArch {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(arch: &str) -> Option<Self> {
        match arch {
            "win32" | "x32" | "32" => Some(Self::Win32),
            "win64" | "x64" | "64" => Some(Self::Win64),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Win32 => "win32",
            Self::Win64 => "win64"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtonLoader {
    /// Set `WINELOADER` variable as binary specified in `Proton` struct
    Current,

    /// Don't set `WINELOADER` variable, so wine will try to use system-wide binary
    Default,

    /// Set custom `WINELOADER` variable
    Custom(PathBuf)
}

impl Default for ProtonLoader {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proton {
    binary: PathBuf,

    /// Specifies `WINEPREFIX` variable
    pub prefix: Option<PathBuf>,

    /// Specifies `WINEARCH` variable
    pub arch: Option<ProtonArch>,

    /// Path to wineboot binary
    pub wineboot: Option<PathBuf>,

    /// Specifies `WINESERVER` variable
    pub wineserver: Option<PathBuf>,

    /// Specifies `WINELOADER` variable
    pub wineloader: ProtonLoader
}

impl Default for Proton {
    fn default() -> Self {
        Self::from_binary("wine")
    }
}

impl Proton {
    pub fn new<T: Into<PathBuf>>(binary: T, prefix: Option<T>, arch: Option<ProtonArch>, wineboot: Option<T>, wineserver: Option<T>, wineloader: ProtonLoader) -> Self {
        Proton {
            binary: binary.into(),
            prefix: prefix.map(|value| value.into()),
            arch,
            wineboot: wineboot.map(|value| value.into()),
            wineserver: wineserver.map(|value| value.into()),
            wineloader
        }
    }

    pub fn from_binary<T: Into<PathBuf>>(binary: T) -> Self {
        Self::new(binary, None, None, None, None, ProtonLoader::default())
    }

    /// Try to get version of provided wine binary. Runs command: `wine --version`
    /// 
    /// TODO: proton has a version file. Use it.
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// match Proton::default().version() {
    ///     Ok(version) => println!("Proton version: {:?}", version),
    ///     Err(err) => eprintln!("Proton is not available: {}", err)
    /// }
    /// ```
    pub fn version(&self) -> Result<OsString> {
        let output = Command::new(&self.binary)
           .arg("--version")
           .stdout(Stdio::piped())
           .stderr(Stdio::null())
           .output()?;

        Ok(OsString::from_vec(output.stdout))
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
    /// assert_eq!(Proton::default().wineboot(), PathBuf::from("wineboot"));
    /// assert_eq!(Proton::from_binary("/wine_build/wine").wineboot(), PathBuf::from("/wine_build/wineboot"));
    /// assert_eq!(Proton::from_binary("/wine_build_without_wineboot/wine").wineboot(), PathBuf::from("wineboot"));
    /// ```
    pub fn wineboot(&self) -> PathBuf {
        self.wineboot.clone().unwrap_or_else(|| self.get_inner_binary("wineboot"))
    }

    /// Get path to wineserver binary, or "wineserver" if not specified
    /// 
    /// If wine binary is specified (so not system), then function will try to find wineserver binary inside of this wine's folder
    /// 
    /// ```no_run
    /// use wincompatlib::prelude::*;
    /// 
    /// use std::path::PathBuf;
    /// 
    /// assert_eq!(Proton::default().wineserver(), PathBuf::from("wineserver"));
    /// assert_eq!(Proton::from_binary("/wine_build/wine").wineserver(), PathBuf::from("/wine_build/wineserver"));
    /// assert_eq!(Proton::from_binary("/wine_build_without_wineserver/wine").wineserver(), PathBuf::from("wineserver"));
    /// ```
    pub fn wineserver(&self) -> PathBuf {
        self.wineserver.clone().unwrap_or_else(|| self.get_inner_binary("wineserver"))
    }

    /// Get path to wine binary, or "wine" if not specified (`ProtonLoader::Default`)
    pub fn wineloader(&self) -> PathBuf {
        match &self.wineloader {
            ProtonLoader::Default => PathBuf::from("wine"),
            ProtonLoader::Current => self.binary.clone(),
            ProtonLoader::Custom(path) => path.clone()
        }
    }

    /// Get environment variables map from current struct's values
    /// 
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// use std::process::Command;
    /// 
    /// let wine = Proton::default().with_arch(ProtonArch::Win64);
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

        if let Some(arch) = self.arch {
            env.insert("WINEARCH", match arch {
                ProtonArch::Win32 => OsString::from("win32"),
                ProtonArch::Win64 => OsString::from("win64")
            });
        }

        if let Some(server) = &self.wineserver {
            env.insert("WINESERVER", server.as_os_str().to_os_string());
        }

        match &self.wineloader {
            ProtonLoader::Default => (),
            ProtonLoader::Current => {
                env.insert("WINELOADER", self.binary.as_os_str().to_os_string());
            },
            ProtonLoader::Custom(path) => {
                env.insert("WINELOADER", path.as_os_str().to_os_string());
            }
        }

        env
    }
}
