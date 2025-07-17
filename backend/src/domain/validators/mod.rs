use regex::Regex;

pub fn validate_username(username: &str) -> Result<(), &'static str> {
    if username.len() < 3 {
        return Err("Username must be at least 3 characters long");
    }
    if username.len() > 20 {
        return Err("Username must be at most 20 characters long");
    }
    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !re.is_match(username) {
        return Err("Username can only contain letters, numbers, and underscores");
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() < 6 {
        return Err("Password must be at least 6 characters long");
    }
    Ok(())
}

pub fn validate_instagram_link(link: &str) -> Result<(), &'static str> {
    if link.is_empty() {
        return Ok(());
    }
    
    // Accept both @username format and full URLs
    let re = Regex::new(r"^(@[a-zA-Z0-9_.]+|https?://(?:www\.)?instagram\.com/[a-zA-Z0-9_.]+/?)$").unwrap();
    if !re.is_match(link) {
        return Err("Invalid Instagram link format");
    }
    Ok(())
}

pub fn validate_time_format(time: &str) -> Result<(), &'static str> {
    let re = Regex::new(r"^([0-1]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap();
    if !re.is_match(time) {
        return Err("Time must be in HH:MM format");
    }
    Ok(())
} 