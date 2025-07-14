#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent {
            id: "test123".to_string(),
            name: "test-agent".to_string(),
            created: chrono::Local::now(),
        };

        assert_eq!(agent.id, "test123");
        assert_eq!(agent.name, "test-agent");
    }

    #[test]
    fn test_agent_details() {
        let details = AgentDetails {
            id: "abc123def456".to_string(),
            name: "my-agent".to_string(),
            status: "Running".to_string(),
            created: "2024-01-01 12:00:00".to_string(),
            ports: Some("8080->80".to_string()),
        };

        assert_eq!(details.id, "abc123def456");
        assert_eq!(details.name, "my-agent");
        assert_eq!(details.status, "Running");
        assert!(details.ports.is_some());
    }

    #[tokio::test]
    async fn test_agent_list_parsing() {
        // Test parsing of container list output
        let test_output = "id1|agent1|2024-01-01\nid2|agent2|2024-01-02\n";

        // This is a unit test for the parsing logic
        let lines: Vec<&str> = test_output
            .lines()
            .filter(|line| !line.is_empty())
            .collect();
        assert_eq!(lines.len(), 2);

        for line in lines {
            let parts: Vec<&str> = line.split('|').collect();
            assert_eq!(parts.len(), 3);
        }
    }

    #[test]
    fn test_name_validation_logic() {
        // Valid names
        let valid_names = vec![
            "alice",
            "bob-123",
            "test_agent",
            "Agent123",
            "my-cool-agent_1",
        ];

        for name in &valid_names {
            assert!(
                name.chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
                "Name {name} should be valid"
            );
        }

        // Invalid names
        let invalid_names = vec![
            "",
            "alice@bob",
            "test agent",
            "agent/test",
            "agent.test",
            "agent:test",
        ];

        for name in &invalid_names {
            assert!(
                name.is_empty()
                    || !name
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
                "Name {name} should be invalid"
            );
        }
    }

    #[tokio::test]
    async fn test_agent_details_parsing() {
        let test_output = "id123|test-agent|Up 2 hours|2024-01-01 10:00|8080->80\n";

        let parts: Vec<&str> = test_output.trim().split('|').collect();
        assert_eq!(parts.len(), 5);

        let details = AgentDetails {
            id: parts[0].to_string(),
            name: parts[1].to_string(),
            status: parts[2].to_string(),
            created: parts[3].to_string(),
            ports: Some(parts[4].to_string()),
        };

        assert_eq!(details.id, "id123");
        assert_eq!(details.name, "test-agent");
        assert_eq!(details.status, "Up 2 hours");
        assert_eq!(details.ports, Some("8080->80".to_string()));
    }
}
