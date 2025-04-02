use crate::fs::{open, OpenOptions, SystemTimeSpec};
use std::fs::FileTimes;
use std::time::SystemTime;
use std::os::windows::fs::OpenOptionsExt;
use std::path::Path;
use std::{fs, io};
use winapi::um::winbase::{FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT};

#[inline]
pub(crate) fn set_times_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    set_times_inner(start, path, atime, mtime, 0)
}

#[inline]
pub(crate) fn set_times_nofollow_impl(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
) -> io::Result<()> {
    set_times_inner(start, path, atime, mtime, FILE_FLAG_OPEN_REPARSE_POINT)
}

fn set_times_inner(
    start: &fs::File,
    path: &Path,
    atime: Option<SystemTimeSpec>,
    mtime: Option<SystemTimeSpec>,
    custom_flags: u32,
) -> io::Result<()> {
    let custom_flags = custom_flags | FILE_FLAG_BACKUP_SEMANTICS;

    let atime_std = atime.map(|ts| match ts.into_std() {
        fs_set_times::SystemTimeSpec::SymbolicNow => SystemTime::now(),
        fs_set_times::SystemTimeSpec::Absolute(t) => t,
    });
    
    let mtime_std = mtime.map(|ts| match ts.into_std() {
        fs_set_times::SystemTimeSpec::SymbolicNow => SystemTime::now(),
        fs_set_times::SystemTimeSpec::Absolute(t) => t,
    });

    // On Windows, `set_times` requires write permissions.
    open(
        start,
        path,
        OpenOptions::new().write(true).custom_flags(custom_flags),
    )?
    .set_times(
        FileTimes::new()
        .set_accessed(atime_std.unwrap())
        .set_modified(mtime_std.unwrap())
    )
}
