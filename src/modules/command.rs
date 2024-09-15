use clap::Parser;

/// Backup to both local and cloud storage.
#[derive(Debug, Parser)]
pub(crate) struct CmdArgs {
    /// Path of the configuration file.
    #[arg(
        short,
        long,
        default_value = "~/app/backup-to-drive/backup-minecraft.toml"
    )]
    pub(crate) config: String,
}
