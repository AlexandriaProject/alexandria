use std::env;
use std::path::PathBuf;
use crate::constants;

pub fn local_dir() -> Result<PathBuf, env::VarError> {
    let mut path: PathBuf;
    #[cfg(target_os = "windows")] {
        path = PathBuf::from(env::var("LOCALAPPDATA")?);
        path.push(constants::PROJECT);
    }
    #[cfg(not(target_os = "windows"))] {
        path = PathBuf::from(env::var("HOME")?);
        path.push(".local/share");
        path.push(constants::PROJECT);
    }
    Ok(path)
}
