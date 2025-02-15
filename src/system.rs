use crate::errors::DirectoryNotFoundError;
use env_home::env_home_dir as home_dir;

pub fn get_user_home() -> Result<String, DirectoryNotFoundError> {
    match home_dir() {
        Some(path) => Result::Ok(path.display().to_string()),
        None => Result::Err(DirectoryNotFoundError {
            message: String::from("Home directory not found"),
            code: 1,
        }),
    }
}

#[cfg(test)]
mod system_tests {
    use super::*;

    #[test]
    fn test_get_user_home() {
        let home = get_user_home();
        assert!(home.is_ok());
    }
}
