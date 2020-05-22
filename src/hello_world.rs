use std::fs::OpenOptions;
use std::io::Write;

pub fn generate_file() -> std::io::Result<()> {
    let mut bla = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("hello.txt")
        .unwrap();
    bla.write(b"Hello world!")?;
    Ok(())
}
