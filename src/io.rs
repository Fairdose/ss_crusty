use std::fs::File;
use std::io::{self, Read, Write};

/// Reads the entire content of a file into a string.
///
/// This is used primarily for loading files that contain lists of URLs.
///
/// # Examples
///
/// ```no_run
/// use io::read_file;
/// let content = read_file("urls.json").unwrap();
/// assert!(content.len() > 0);
/// ```
pub fn read_file(path: &str) -> io::Result<String> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}

/// Writes a string to a file, creating or truncating it.
///
/// This is used to output the final, serialized JSON results.
///
/// # Examples
///
/// ```no_run
/// use io::write_file;
/// write_file("output.json", "{ \"key\": \"value\" }").unwrap();
/// ```
pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())
}
