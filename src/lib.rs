use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn setup<P>(command: &str, filename: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    setup_impl(command, filename.as_ref())
}

fn setup_impl(command: &str, filename: &Path) -> io::Result<()> {
    let home_dir = PathBuf::from(env!("HOME"));
    let man1_dir = home_dir.join(".local/share/man/man1");
    if !man1_dir.is_dir() {
        fs::create_dir_all(&man1_dir)?;
    }
    let mut manual = man1_dir.join(command);
    manual.set_extension("1");
    fs::copy(filename, manual)?;
    Ok(())
}
