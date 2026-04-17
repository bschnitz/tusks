use std::char;

#[derive(Debug)]
pub struct RenderConfig {
    pub min_gap: usize,
    pub task_indent: usize,
    pub header_indent: usize,
    pub span_token: char,
    pub use_colors: bool
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            min_gap: 4,
            task_indent: 4,
            header_indent: 2,
            span_token: '.',
            use_colors: true
        }
    }
}

#[derive(Debug)]
pub struct ListGroupHeader {
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct ListTask {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct ListGroup {
    pub header: ListGroupHeader,
    pub tasks: Vec<ListTask>,
}

#[derive(Debug)]
pub struct List {
    pub description: Option<String>,
    pub groups: Vec<ListGroup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_config_defaults() {
        let config = RenderConfig::default();
        assert_eq!(config.min_gap, 4);
        assert_eq!(config.task_indent, 4);
        assert_eq!(config.header_indent, 2);
        assert_eq!(config.span_token, '.');
        assert!(config.use_colors);
    }
}
