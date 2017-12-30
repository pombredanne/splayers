extern crate ar;
extern crate bzip2;

#[macro_use]
extern crate error_chain;

#[cfg(intellij_type_hinting)]
extern crate error_chain_for_dumb_ides;

extern crate flate2;
extern crate tar;
extern crate tempdir;
extern crate time as crates_time;

#[macro_use]
extern crate more_asserts;

extern crate xz2;
extern crate zip;

use std::path::Path;
use std::path::PathBuf;

mod errors;
mod temps;
mod file_type;
mod meta;
mod mio;
mod simple_time;
mod unpacker;

pub use errors::*;
pub use unpacker::Status;

pub struct Unpack {
    status: Status,
    dir: tempdir::TempDir,
}

impl Unpack {
    pub fn unpack_into<P: AsRef<Path>, F: AsRef<Path>>(what: F, root: P) -> Result<Unpack> {
        let mut temps = temps::Temps::new_in(root)?;
        Ok(Unpack {
            status: unpacker::unpack_unknown(mio::Mio::from_path(what)?, &mut temps),
            dir: temps.into_dir(),
        })
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    /// causes the temporary files to not be deleted
    pub fn into_path(self) -> PathBuf {
        self.dir.into_path()
    }
}

pub fn print(entries: &[unpacker::Entry], depth: usize) {
    for entry in entries {
        print!(
            "{} - {:?} at {:?}:",
            String::from_utf8_lossy(&vec![b' '; depth]),
            String::from_utf8_lossy(&entry.local.path),
            entry.local.temp
        );

        if let unpacker::Status::Success(ref children) = entry.children {
            println!();
            print(children, depth + 2);
        } else {
            println!(" {:?}", entry.children);
        }
    }
}
