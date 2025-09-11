use std::{
    ffi::OsStr,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

use crate::Bundle;

use super::{BundleUnzipError, BundleZipError};

/// Compresses a directory into a ZIP archive.
///
/// This function creates a ZIP archive from a directory, preserving the directory structure.
/// It's commonly used for packaging plugin bundles.
///
/// # Parameters
///
/// * `path` - Path to the directory to compress
/// * `target_path` - Directory where the ZIP file will be created
/// * `compression_method` - Compression method to use (e.g., Stored, Deflated)
/// * `callback` - Optional callback function called for each processed file/directory
///
/// # Returns
///
/// Returns `Result<(), BundleZipError>` indicating success or failure.
///
/// # Type Parameters
///
/// * `S` - Type that can be converted to OsStr (for the source path)
/// * `F` - Callback function type that takes a `&Path`
///
/// # Example
///
/// ```rust,no_run
/// use plux_rs::utils::archive::zip;
/// use std::path::Path;
/// use zip::CompressionMethod;
///
/// // Compress a plugin directory
/// zip(
///     "path/to/plugin_directory",
///     "output/directory",
///     CompressionMethod::Deflated,
///     Some(|path: &Path| println!("Processing: {}", path.display()))
/// )?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn zip<S, F>(
    path: &S,
    target_path: &str,
    compression_method: zip::CompressionMethod,
    mut callback: Option<F>,
) -> Result<(), BundleZipError>
where
    S: AsRef<OsStr> + ?Sized,
    F: FnMut(&Path),
{
    let path = Path::new(path);
    let target_path =
        Path::new(target_path).join(path.file_name().ok_or(BundleZipError::NoNameFailed)?);

    if !path.is_dir() {
        return Err(BundleZipError::MissingBundleFailed);
    }

    match target_path.exists() {
        true if target_path.is_file() => Ok(()),
        true => Err(BundleZipError::ContainSameDirFailed),
        false => Ok({
            let file =
                File::create(target_path).map_err(|e| BundleZipError::CreateBundleFailed(e))?;
            let mut archive = ZipWriter::new(file);
            let options = SimpleFileOptions::default()
                .compression_method(compression_method)
                .unix_permissions(0o755);

            let mut buffer = Vec::new();
            for entry in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let entry_path = entry.path();
                let name = entry_path.strip_prefix(path).unwrap();

                if entry_path.is_file() {
                    #[allow(deprecated)]
                    archive.start_file_from_path(name, options)?;
                    let mut f = File::open(entry_path)?;

                    f.read_to_end(&mut buffer)?;
                    archive.write_all(&buffer)?;
                    buffer.clear();
                } else if !name.as_os_str().is_empty() {
                    #[allow(deprecated)]
                    archive.add_directory_from_path(name, options)?;
                }

                callback.as_mut().map(|callback| callback(name));
            }
        }),
    }
}

/// Extracts a ZIP archive to a directory.
///
/// This function extracts a ZIP archive and creates a Bundle from the extracted directory.
/// It's commonly used for unpacking plugin bundles.
///
/// # Parameters
///
/// * `path` - Path to the ZIP file to extract
/// * `target_path` - Directory where the archive will be extracted
///
/// # Returns
///
/// Returns `Result<Bundle, BundleUnzipError>` containing the bundle information
/// from the extracted directory on success.
///
/// # Type Parameters
///
/// * `S` - Type that can be converted to OsStr (for the archive path)
///
/// # Example
///
/// ```rust,no_run
/// use plux_rs::utils::archive::unzip;
///
/// // Extract a plugin bundle
/// let bundle = unzip("path/to/plugin.zip", "output/directory")?;
/// println!("Extracted plugin: {}", bundle.id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn unzip<S>(path: &S, target_path: &str) -> Result<Bundle, BundleUnzipError>
where
    S: AsRef<OsStr> + ?Sized,
{
    let path = Path::new(path);
    let target_path =
        Path::new(target_path).join(path.file_name().ok_or(BundleUnzipError::NoNameFailed)?);

    if !path.is_file() {
        return Err(BundleUnzipError::MissingBundleFailed);
    }

    match target_path.exists() {
        true if target_path.is_dir() => Ok(()),
        true => Err(BundleUnzipError::ContainSameFileFailed),
        false => Ok({
            let file = File::open(path)?;
            let mut archive = ZipArchive::new(file)?;
            archive.extract(&target_path)?;
        }),
    }?;

    Ok(Bundle::from_filename(target_path.file_name().unwrap())?)
}

#[test]
fn test_zip() {
    let temp_path = "./bundles/temp";
    if !Path::new(temp_path).exists() {
        std::fs::create_dir_all(temp_path).unwrap();
    }

    let name = "plugin_a-v1.0.0.vpl";
    let path = format!("./bundles/{name}");

    let target_path = temp_path;
    zip(
        &path,
        target_path,
        zip::CompressionMethod::Stored,
        Some(|name: &Path| println!("{}", name.display())),
    )
    .unwrap();

    std::fs::remove_file(format!("{target_path}/{name}")).unwrap();
}

#[test]
fn test_unzip() {
    let temp_path = "./bundles/temp";
    if !Path::new(temp_path).exists() {
        std::fs::create_dir_all(temp_path).unwrap();
    }

    let name = "plugin_b-v1.0.0.vpl";
    let path = format!("./bundles/{name}");

    let target_path = temp_path;
    let bundle = unzip(&path, target_path).unwrap();
    println!("{bundle}");

    std::fs::remove_dir_all(format!("{target_path}/{name}")).unwrap();
}
