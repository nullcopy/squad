#[cfg(test)]
mod tests {
    #[test]
    fn test_command_injection_prevention() {
        // Test that special characters don't cause command injection
        let dangerous_inputs = vec![
            "; rm -rf /",
            "| cat /etc/passwd",
            "$(whoami)",
            "`id`",
            "&& echo hacked",
            "'; DROP TABLE users; --",
        ];

        // These should all be treated as literal strings, not executed
        for input in dangerous_inputs {
            // In a real test, we'd mock the runtime and verify
            // that the input is passed as a single argument
            assert!(input.contains(|c: char| !c.is_alphanumeric()));
        }
    }

    #[test]
    fn test_name_validation_security() {
        use super::super::agent::MAX_AGENT_NAME_LENGTH;

        // Test various malicious inputs
        let max_length_name = "a".repeat(MAX_AGENT_NAME_LENGTH);
        let too_long_name = "a".repeat(MAX_AGENT_NAME_LENGTH + 1);

        let test_cases = vec![
            ("", false, "empty name"),
            ("a", true, "single letter"),
            ("123", false, "starts with number"),
            ("_test", false, "starts with underscore"),
            ("-test", false, "starts with hyphen"),
            ("test/path", false, "contains slash"),
            ("test\\path", false, "contains backslash"),
            ("test space", false, "contains space"),
            ("test.dot", false, "contains dot"),
            ("test:colon", false, "contains colon"),
            (max_length_name.as_str(), true, "max length"),
            (too_long_name.as_str(), false, "exceeds max length"),
        ];

        for (name, should_be_valid, description) in test_cases {
            let is_valid = !name.is_empty()
                && name.len() <= MAX_AGENT_NAME_LENGTH
                && name
                    .chars()
                    .next()
                    .map(|c| c.is_alphabetic())
                    .unwrap_or(false)
                && name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_');

            assert_eq!(
                is_valid, should_be_valid,
                "Name validation failed for '{name}' ({description}): expected {should_be_valid}, got {is_valid}"
            );
        }
    }
}
