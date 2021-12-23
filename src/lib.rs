use std::env;
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
    let home_directory = home_dir().expect("can't find $HOME directory");
    let man_directory = home_directory.join(".local/share/man/man1");
    if !man_directory.is_dir() {
        fs::create_dir_all(&man_directory)?;
    }
    let mut manual = man_directory.join(command);
    manual.set_extension("1");
    fs::copy(filename, manual)?;
    Ok(())
}

#[inline]
fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}
