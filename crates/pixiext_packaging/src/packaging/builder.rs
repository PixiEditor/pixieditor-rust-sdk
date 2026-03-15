use std::process::Command;
use base64::prelude::*;
use crate::packaging::metadata::ExtensionMetadata;
use std::fs;
use std::io::{self, Read, Seek, Write};
use std::path::{Path, PathBuf};
use rand::RngCore;
use walkdir::WalkDir;
use crate::packaging::encryptor::encrypt_resources_with_exe;
use crate::packaging::utils::zip_dir;

pub fn build_pixiext() -> anyhow::Result<()> {

    println!("Creating build directory...");

    if fs::exists("build")? {
        fs::remove_dir_all("build")?;
    }

    fs::create_dir_all("build")?;

    println!("Copying extension.json...");

    fs::copy("extension.json", "build/extension.json")?;

    println!("Copying resources and localization...");

    copy_dir_recursive("Resources", "build/Resources")?;
    copy_dir_recursive("Localization", "build/Localization")?;

    println!("Encrypting resources...");

    let mut key = [0u8; 16];
    let mut iv = [0u8; 16];

    rand::thread_rng().fill_bytes(&mut key);
    rand::thread_rng().fill_bytes(&mut iv);

    let base64_key = BASE64_STANDARD.encode(&key);
    let base64_iv = BASE64_STANDARD.encode(&iv);

    encrypt_resources_with_exe("build/Resources", "build", "build", &base64_key, &base64_iv)?;
    fs::remove_dir_all("build/Resources")?;
    fs::create_dir("build/Resources")?;
    fs::copy("build/resources.data", "build/Resources/resources.data")?;
    fs::remove_file("build/resources.data")?;
    fs::remove_file("build/resources.zip")?;
    write_crypto_file(&key, &iv)?;

    println!("Compiling the project");

    build_wasm()?;

    let wasm = WalkDir::new("target/wasm32-wasip1/release")
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.path().extension().and_then(|e| e.to_str()) == Some("wasm"));

    if let Some(entry) = wasm {
        println!("{}", entry.path().display());
        fs::copy(
            entry.path(),
            "build/extension.wasm",
        )?;
    }
    else {
        anyhow::bail!("Compiled wasm file not found");
    }



    println!("Creating .pixiext package...");

    create_package("build", "dist")?;

    println!("Build successful!");
    Ok(())
}

fn build_wasm() -> anyhow::Result<()> {

    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--target",
            "wasm32-wasip1"
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("cargo build failed");
    }

    Ok(())
}

fn write_crypto_file(key: &[u8], iv: &[u8]) -> anyhow::Result<()> {

    let key_str = key
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let iv_str = iv
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let content = format!(
        r#"
pub static KEY: [u8; 16] = [{key_str}];
pub static IV: [u8; 16] = [{iv_str}];
"#
    );

    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output_path = Path::new(cargo_manifest_dir)
    .join("../pixieditor_sdk/src/generated_crypto.rs");
    fs::write(output_path, content)?;

    Ok(())
}

pub fn copy_dir_recursive(src: &str, dst: &str) -> anyhow::Result<()> {

    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {

        let entry = entry?;
        let path = entry.path();

        let target = std::path::Path::new(dst)
            .join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(path.to_str().unwrap(), target.to_str().unwrap())?;
        } else {
            std::fs::copy(path, target)?;
        }

    }

    Ok(())
}


pub fn create_package(build_dir: &str, dist_dir: &str) -> anyhow::Result<()> {

    let metadata: ExtensionMetadata =
        serde_json::from_str(
            &std::fs::read_to_string(format!("{build_dir}/extension.json"))?
        )?;

    std::fs::create_dir_all(dist_dir)?;

    let package_path =
        format!("{dist_dir}/{}.pixiext", metadata.uniqueName);

    let file = std::fs::File::create(package_path)?;

    zip_dir(Path::new(build_dir), file, zip::CompressionMethod::Deflated)?;

    Ok(())
}