pub const VERSION: &str = "0.1.3";

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn exposes_workspace_version() {
        assert!(!VERSION.is_empty());
    }
}
