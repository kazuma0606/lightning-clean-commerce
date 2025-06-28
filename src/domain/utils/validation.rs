pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8
}

pub fn validate_username(username: &str) -> bool {
    username.len() >= 3 && username.len() <= 50
}
