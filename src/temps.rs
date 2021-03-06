use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use failure::Error;
use failure::ResultExt;
use tempfile::TempDir;

#[derive(Debug)]
pub struct Temps {
    dir: TempDir,
    count: usize,
}

impl Temps {
    pub fn new_in<P: AsRef<Path>>(inside: P) -> io::Result<Self> {
        Ok(Temps {
            dir: TempDir::new_in(inside)?,
            count: 0,
        })
    }

    pub fn insert<R: Read>(&mut self, mut from: R) -> Result<PathBuf, Error> {
        let mut dest = self.dir.as_ref().to_path_buf();
        let three_hex_digits = 4096;
        let subdir = self.count / three_hex_digits;
        let in_dir = self.count % three_hex_digits;
        dest.push(format!("{}", subdir));
        if 0 == in_dir {
            fs::create_dir(&dest).expect("tempdir");
        }
        dest.push(format!("{:03x}.tmp", in_dir));

        self.count += 1;

        let mut tmp = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&dest)
            .with_context(|_| format_err!("creating {:?}", dest))?;

        loop {
            let mut buf = [0u8; 8 * 1024];
            let found = from.read(&mut buf)?;
            if 0 == found {
                break;
            }
            tmp.write_all(&buf[..found]).expect("writing to temp file");
        }

        Ok(dest)
    }

    pub fn into_dir(self) -> TempDir {
        self.dir
    }
}
