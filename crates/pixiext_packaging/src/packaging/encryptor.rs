use std::process::Command;
use std::path::{Path, PathBuf};
use anyhow::Context;

pub fn encrypt_resources_with_exe(
    resources_dir: &str,
    intermediate_dir: &str,
    output_dir: &str,
    key_base64: &String,
    iv_base64: &String,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(intermediate_dir)
        .context("Failed to create intermediate directory")?;

    std::fs::create_dir_all(output_dir)
        .context("Failed to create output directory")?;

    let exe_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("binaries/ResourceEncryptor.exe");

    if !exe_path.exists() {
        anyhow::bail!("ResourceEncryptor.exe not found at {:?}", exe_path);
    }
    let mut cmd = Command::new(exe_path);
    cmd.arg(resources_dir)
        .arg(intermediate_dir)
        .arg(output_dir)
        .arg(key_base64)
        .arg(iv_base64);

    let status = cmd.status()
        .context("Failed to start ResourceEncryptor.exe")?;

    if !status.success() {
        anyhow::bail!("ResourceEncryptor.exe exited with code {}", status.code().unwrap_or(-1));
    }

    Ok(())
}