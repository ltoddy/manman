use std::ffi::{CStr, OsString};
use std::fs;
use std::mem;
use std::os::unix::prelude::*;
use std::path::{Path, PathBuf};
use std::ptr;

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
    let manual = man_directory.join(command);
    fs::copy(filename, manual)?;
    Ok(())
}

fn home_dir() -> Option<PathBuf> {
    return std::env::var_os("HOME")
        .or_else(|| unsafe { fallback() })
        .map(PathBuf::from);

    #[cfg(any(
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten",
        target_os = "redox",
        target_os = "vxworks",
        target_os = "espidf"
    ))]
    unsafe fn fallback() -> Option<OsString> {
        None
    }
    #[cfg(not(any(
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten",
        target_os = "redox",
        target_os = "vxworks",
        target_os = "espidf"
    )))]
    unsafe fn fallback() -> Option<OsString> {
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512_usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = mem::zeroed();
        let mut result = ptr::null_mut();
        match libc::getpwuid_r(
            libc::getuid(),
            &mut passwd,
            buf.as_mut_ptr(),
            buf.capacity(),
            &mut result,
        ) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_dir as *const _;
                let bytes = CStr::from_ptr(ptr).to_bytes().to_vec();
                Some(OsStringExt::from_vec(bytes))
            }
            _ => None,
        }
    }
}
