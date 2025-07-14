#[cfg(test)]
mod tests {
    use crate::container::parser::parse_container_output;

    #[test]
    fn test_parse_container_output_empty() {
        let output = "";
        let result = parse_container_output(output, |_| Some("test"));
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_container_output_single_line() {
        let output = "field1|field2|field3";
        let result = parse_container_output(output, |parts| {
            if parts.len() == 3 {
                Some(format!("{}-{}-{}", parts[0], parts[1], parts[2]))
            } else {
                None
            }
        });
        assert_eq!(result, vec!["field1-field2-field3"]);
    }

    #[test]
    fn test_parse_container_output_multiple_lines() {
        let output = "a|b|c\nd|e|f\ng|h|i";
        let result = parse_container_output(output, |parts| {
            if parts.len() == 3 {
                Some(parts[0].to_string())
            } else {
                None
            }
        });
        assert_eq!(result, vec!["a", "d", "g"]);
    }

    #[test]
    fn test_parse_container_output_skip_empty_lines() {
        let output = "a|b|c\n\nd|e|f\n\n";
        let result = parse_container_output(output, |parts| {
            if parts.len() == 3 {
                Some(parts[0].to_string())
            } else {
                None
            }
        });
        assert_eq!(result, vec!["a", "d"]);
    }

    #[test]
    fn test_parse_container_output_filter_invalid() {
        let output = "a|b|c\ninvalid\nd|e|f";
        let result = parse_container_output(output, |parts| {
            if parts.len() == 3 {
                Some(parts[0].to_string())
            } else {
                None
            }
        });
        assert_eq!(result, vec!["a", "d"]);
    }

    #[test]
    fn test_parse_container_output_complex_parser() {
        let output = "alice|running|123\nbob|stopped|456";

        #[derive(Debug, PartialEq)]
        struct Container {
            name: String,
            status: String,
            id: String,
        }

        let result = parse_container_output(output, |parts| {
            if parts.len() == 3 {
                Some(Container {
                    name: parts[0].to_string(),
                    status: parts[1].to_string(),
                    id: parts[2].to_string(),
                })
            } else {
                None
            }
        });

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "alice");
        assert_eq!(result[0].status, "running");
        assert_eq!(result[0].id, "123");
        assert_eq!(result[1].name, "bob");
        assert_eq!(result[1].status, "stopped");
        assert_eq!(result[1].id, "456");
    }
}
