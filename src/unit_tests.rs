use super::*;
use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;

#[test]
fn empty_test() {}

struct ScopedFile
{
    uri: PathBuf,
}

impl ScopedFile
{
    pub fn new(fq_filename: PathBuf) -> GeneralResult<ScopedFile> {
        ScopedFile::from_string(fq_filename, "")
    }

    pub fn from_string(fq_filename: PathBuf, file_contents: &str) -> GeneralResult<ScopedFile> {
        OpenOptions::new().read(true)
                          .write(true)
                          .create_new(true)
                          .mode(0o744)  //without executable set, any attempt to execute will panic
                          .open(&fq_filename)?
            .write_all(file_contents.as_bytes())?;
        Ok(ScopedFile { uri: fq_filename, })
    }
}

impl Drop for ScopedFile
{
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.uri);
    }
}

#[test]
fn calling_lib_main_succeeds()
{
    let fh = ScopedFile::new(PathBuf::from(MEMORY_MAP_DEVICE_FILENAME_VALUE));
    let result = lib_main(Vec::<String>::new());
    println!("lib_main() result: {:?}", result);
    assert!(result.is_ok());
}
