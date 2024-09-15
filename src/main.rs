use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use resolve_path::PathResolveExt;

pub mod modules;
use modules::*;

fn main() -> Result<()> {
    let cmd_args = CmdArgs::parse();

    let config_path = cmd_args.config.try_resolve()?;
    let config_text = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_text)?;

    let target_path = config.path.target.try_resolve()?;
    let local_prefix_path = config.path.local.try_resolve()?;
    let cloud_prefix_path = config.path.cloud.try_resolve()?;

    println!(
        "\
Target: {}
Local: {}
Cloud: {}
",
        target_path.to_string_lossy(),
        local_prefix_path.to_string_lossy(),
        cloud_prefix_path.to_string_lossy()
    );

    let target_filename = target_path
        .file_name()
        .context(format!(
            "Cannot get the last component from {}.",
            target_path.to_string_lossy()
        ))?
        .to_string_lossy();

    let date = chrono::Local::now().format("%Y%m%d").to_string();
    let basename = format!("{}_{}", target_filename, date);

    // Path of the zip file for local storage.
    let local_zip_path = generate_unique_zip_path(&local_prefix_path, &basename);
    let local_storage_dir = local_zip_path.parent().unwrap();
    if !local_storage_dir.exists() {
        fs::create_dir_all(local_storage_dir)?;
        println!(
            "Created directory `{}`.",
            local_storage_dir.to_string_lossy()
        );
    }
    create_local_zip(&target_path, &local_zip_path)?;
    println!(
        "\nCreated zip file in `{}`.",
        local_zip_path.to_string_lossy()
    );

    // Deleted existing zip file(s) on cloud storage.
    let deleted_filenames = delete_cloud_zip(&cloud_prefix_path, &target_filename)?;
    println!(
        "\nDeleted {} zip file(s) from {}.",
        deleted_filenames.len(),
        cloud_prefix_path.to_string_lossy()
    );
    for filename in &deleted_filenames {
        println!("  Deleted `{}`.", filename);
    }

    // Path of the zip file for cloud storage.
    let cloud_zip_path = cloud_prefix_path.join(local_zip_path.file_name().unwrap());
    let cloud_storage_dir = cloud_zip_path.parent().unwrap();
    if !cloud_storage_dir.exists() {
        fs::create_dir_all(cloud_storage_dir)?;
        println!(
            "Created directory `{}`.",
            cloud_storage_dir.to_string_lossy()
        );
    }
    fs::copy(&local_zip_path, &cloud_zip_path)?;
    println!(
        "\nCopied zip file to `{}`.",
        cloud_zip_path.to_string_lossy()
    );

    Ok(())
}
