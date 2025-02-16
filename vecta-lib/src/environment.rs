use crate::system::get_user_home;
pub struct Environment {
    pub user_home: String,
}

pub fn initialize_environment() -> Environment {
    Environment {
        user_home: get_user_home().unwrap_or_else(|_| String::from(".")),
    }
}
