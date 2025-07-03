use anyhow::Result;
use directories::ProjectDirs;
use std::path::PathBuf;
use std::fs;

pub fn get_config_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "admin-assist", "AdminAssist")
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    
    Ok(proj_dirs.config_dir().to_path_buf())
}

pub fn ensure_config_dir() -> Result<()> {
    let config_dir = get_config_dir()?;
    fs::create_dir_all(&config_dir)?;
    Ok(())
}