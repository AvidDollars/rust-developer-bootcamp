use std::fs;
use std::io::{self, ErrorKind};

pub fn create_missing_folders(paths: &[&str]) -> io::Result<()> {
    for path in paths {
        let folder_creation = match fs::create_dir(path) {
            Err(error) => Err(error),
            Ok(_) => Ok(()),
        };

        if let Err(error) = folder_creation {
            match error.kind() {
                ErrorKind::AlreadyExists => (),
                other_error => return Err(io::Error::from(other_error)),
            }
        }
    }
    Ok(())
}
