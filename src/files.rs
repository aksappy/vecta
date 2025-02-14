use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

pub struct Files;

impl Files {
    /// Reads the content of a file into a String.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the file content as a String or an `Error` if something goes wrong.
    pub fn read_file_to_string(filepath: &str) -> Result<String, Error> {
        let path = Path::new(filepath);

        // Check if the file exists
        if !path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "File not found"));
        }

        // Check if the file is readable
        let file = File::open(path)?; // The ? operator propagates errors

        let mut file = file; // Make file mutable to read from it
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    /// Reads the content of a file into a byte vector.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the file content as a byte vector or an `Error` if something goes wrong.
    pub fn read_file_to_bytes(filepath: &str) -> Result<Vec<u8>, Error> {
        let path = Path::new(filepath);

        // Check if the file exists
        if !path.exists() {
            return Err(Error::new(ErrorKind::NotFound, "File not found"));
        }

        let file = File::open(path)?;
        let mut file = file;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        Ok(contents)
    }

    /// Writes a String to a file.  Creates the file if it doesn't exist, overwrites if it does.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the file.
    /// * `content` - The String to write.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `Error` if something goes wrong.
    pub fn write_string_to_file(filepath: &str, content: &str) -> Result<(), Error> {
        let path = Path::new(filepath);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true) // Create if it doesn't exist
            .truncate(true) // Overwrite if it exists
            .open(path)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Writes a byte array to a file. Creates the file if it doesn't exist, overwrites if it does.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the file.
    /// * `content` - The byte array to write.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `Error` if something goes wrong.
    pub fn write_bytes_to_file(filepath: &str, content: &[u8]) -> Result<(), Error> {
        let path = Path::new(filepath);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_read_and_write_string() -> Result<(), Error> {
        let filepath = "test_string_file.txt";
        let content = "Hello, world! This is a test.";

        Files::write_string_to_file(filepath, content)?;
        let read_content = Files::read_file_to_string(filepath)?;

        assert_eq!(content, read_content);

        fs::remove_file(filepath)?; // Clean up

        Ok(())
    }

    #[test]
    fn test_read_and_write_bytes() -> Result<(), Error> {
        let filepath = "test_bytes_file.txt";
        let content = b"This is a byte array test.";

        Files::write_bytes_to_file(filepath, content)?;
        let read_content = Files::read_file_to_bytes(filepath)?;

        assert_eq!(content, &read_content[..]); // Compare slices

        fs::remove_file(filepath)?; // Clean up

        Ok(())
    }

    #[test]
    fn test_file_not_found() {
        let filepath = "nonexistent_file.txt";
        let result = Files::read_file_to_string(filepath);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }
}
