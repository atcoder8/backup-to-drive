use serde::Deserialize;

/// Configuration for backup.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Config {
    /// Configuration of paths.
    pub(crate) path: ConfigPath,
}

/// Configuration of paths.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ConfigPath {
    /// File or directory to be backed up.
    pub(crate) target: String,

    /// Local backup destination directory.
    pub(crate) local: String,

    /// # Cloud backup destination directory.
    pub(crate) cloud: String,
}
