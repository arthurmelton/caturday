use std::fs::File;
use std::io::Write;

pub fn set_cont(loc: String, cont: String) -> std::io::Result<()> {
    let mut file = File::create(loc)?;
    file.write_all(cont.as_bytes())?;
    Ok(())
}