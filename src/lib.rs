use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub fn setup<P>(command: &str, filename: P) -> Result<()>
where
    P: AsRef<Path>,
{
    setup_impl(command, filename.as_ref())
}

fn setup_impl(command: &str, filename: &Path) -> Result<()> {
    let home_directory = home_dir().unwrap_or_else(|| PathBuf::from("."));
    let man_directory = home_directory.join(".local/share/man/man1");
    if !man_directory.is_dir() {
        fs::create_dir_all(&man_directory)?;
    }
    let mut manual = man_directory.join(command);
    manual.set_extension("1");
    fs::copy(filename, manual)?;
    Ok(())
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}
