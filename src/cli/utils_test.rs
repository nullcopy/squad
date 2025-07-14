#[cfg(test)]
mod tests {
    use crate::cli::utils::{print_error, print_success, print_warning};

    #[test]
    fn test_print_success_with_details() {
        // We can't easily capture stdout in unit tests, so we test the function exists
        // and doesn't panic. In a real scenario, you might use a mock or capture mechanism.
        print_success("Created", "alice", Some("container-id-123"));
        print_success("Started", "bob", Some("port 8080"));
    }

    #[test]
    fn test_print_success_without_details() {
        print_success("Removed", "alice", None);
        print_success("Stopped", "bob", None);
    }

    #[test]
    fn test_print_error() {
        print_error("create", "alice", "container runtime not found");
        print_error("remove", "bob", "container not found");
    }

    #[test]
    fn test_print_warning() {
        print_warning("No agents currently running");
        print_warning("Container runtime may not be available");
    }

    #[test]
    fn test_formatting_functions_dont_panic() {
        // Test various edge cases to ensure functions don't panic
        print_success("", "", None);
        print_success("action", "", Some(""));
        print_error("", "", "");
        print_warning("");

        // Test with special characters
        print_success("create", "agent-with-hyphens_and_underscores", None);
        print_error(
            "action",
            "name with spaces",
            "error with special chars: !@#$%",
        );
        print_warning("Warning with unicode: 🚀 ✓ ✗");
    }

    #[test]
    fn test_all_functions_handle_unicode() {
        print_success("创建", "alice", Some("详情"));
        print_error("错误", "bob", "失败原因");
        print_warning("警告消息");
    }
}
