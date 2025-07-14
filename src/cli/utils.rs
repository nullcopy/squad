use colored::*;

/// Print a success message with consistent formatting
pub fn print_success(action: &str, name: &str, details: Option<&str>) {
    if let Some(details) = details {
        println!(
            "{} {} '{}' with {}",
            "✓".green(),
            action,
            name.cyan(),
            details
        );
    } else {
        println!("{} {} '{}'", "✓".green(), action, name.cyan());
    }
}

/// Print an error message with consistent formatting
pub fn print_error(action: &str, name: &str, error: &str) {
    eprintln!(
        "{} Failed to {} '{}': {}",
        "✗".red(),
        action,
        name.red(),
        error
    );
}

/// Print a warning message with consistent formatting
pub fn print_warning(message: &str) {
    println!("{}", message.yellow());
}
