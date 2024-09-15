use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
    process,
};

pub(crate) mod command;
pub(crate) mod config;

use anyhow::ensure;
pub(crate) use command::CmdArgs;
pub(crate) use config::Config;
use regex::Regex;

pub(crate) fn generate_unique_zip_path<P, S>(prefix: &P, basename: &S) -> PathBuf
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let prefix = prefix.as_ref();
    let base_name = basename.as_ref();

    let zip_path = prefix.join(format!("{}.zip", base_name));
    if !zip_path.exists() {
        return zip_path;
    }

    (2_u32..)
        .find_map(|number| {
            let zip_path = prefix.join(format!("{}_{}.zip", base_name, number));

            if !zip_path.exists() {
                Some(zip_path)
            } else {
                None
            }
        })
        .unwrap()
}

pub(crate) fn create_local_zip<P, Q>(target_path: P, zip_path: Q) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let target_path = target_path.as_ref();
    let zip_path = zip_path.as_ref();

    let output = process::Command::new("zip")
        .arg("-r")
        .args([zip_path, target_path])
        .spawn()?
        .wait_with_output()?;

    ensure!(
        output.status.success(),
        "Failed to create zip file to {}.",
        zip_path.to_string_lossy()
    );

    Ok(())
}

pub(crate) fn delete_cloud_zip<P, S>(
    cloud_prefix_path: P,
    basename: &S,
) -> anyhow::Result<Vec<String>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let cloud_prefix_path = cloud_prefix_path.as_ref();
    let basename = basename.as_ref();

    let re = Regex::new(&format!("^{}.*\\.zip$", basename))?;

    let entries = cloud_prefix_path
        .read_dir()?
        .collect::<io::Result<Vec<DirEntry>>>()?;

    let mut deleted_filenames = vec![];
    for entry in entries {
        let filename = entry.file_name().to_string_lossy().to_string();
        if re.is_match(&filename) {
            fs::remove_file(entry.path())?;
            deleted_filenames.push(filename);
        }
    }

    Ok(deleted_filenames)
}
