/// Parse container output into structured data
pub fn parse_container_output<T, F>(output: &str, parser: F) -> Vec<T>
where
    F: Fn(&[&str]) -> Option<T>,
{
    output
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            parser(&parts)
        })
        .collect()
}
