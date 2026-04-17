#[derive(Debug)]
#[derive(Default)]
pub struct TusksAttr {
    pub debug: bool,
    pub root: bool,
    pub derive_debug_for_parameters: bool,
    pub tasks: Option<TasksConfig>,
}

#[derive(Debug)]
pub struct TasksConfig {
    pub max_groupsize: usize,
    pub max_depth: usize,
    pub separator: String,
    pub use_colors: bool,
}

impl Default for TasksConfig {
    fn default() -> Self {
        Self {
            max_groupsize: 5,
            max_depth: 20,
            separator: ".".to_string(),
            use_colors: true
        }
    }
}

