use indexmap::IndexMap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub path: Vec<String>,
    pub description: Option<String>,
    pub hidden: bool
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        for (a, b) in self.path.iter().zip(other.path.iter()) {
            match a.cmp(b) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }
        self.path.len().cmp(&other.path.len())
    }
}

#[derive(Debug, Default)]
pub struct TaskCollection {
    pub tasks: Vec<Task>,
}

impl TaskCollection {
    pub fn new() -> Self {
        TaskCollection { tasks: Vec::new() }
    }

    pub fn from_tasks(tasks: Vec<Task>) -> Self {
        TaskCollection { tasks }
    }

    pub fn insert_task_sorted(&mut self, new_task: Task) {
        let pos = self.tasks
            .binary_search_by(|task| task.cmp(&new_task))
            .unwrap_or_else(|e| e);
        self.tasks.insert(pos, new_task);
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn into_tasks(self) -> Vec<Task> {
        self.tasks
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

#[derive(Debug)]
pub struct TaskGroup {
    pub direct_tasks: TaskCollection,
    pub subgroups: IndexMap<String, TaskGroup>,
}

impl TaskCollection {
    pub fn get_visible_tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter().filter(|t| !t.hidden)
    }

    pub fn get_number_of_visible_tasks(&self) -> usize {
        self.tasks.iter().filter(|t| !t.hidden).count()
    }
}

#[derive(Debug)]
pub struct TaskList {
    pub description: Option<String>,
    pub max_groupsize: usize,
    pub max_depth: usize,
    pub separator: String,
    pub root: TaskGroup,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(path: &[&str], desc: Option<&str>) -> Task {
        Task {
            path: path.iter().map(|s| s.to_string()).collect(),
            description: desc.map(|s| s.to_string()),
            hidden: false,
        }
    }

    fn hidden_task(path: &[&str]) -> Task {
        Task {
            path: path.iter().map(|s| s.to_string()).collect(),
            description: None,
            hidden: true,
        }
    }

    // --- Task ordering ---

    #[test]
    fn task_ordering_simple() {
        let a = task(&["alpha"], None);
        let b = task(&["beta"], None);
        assert!(a < b);
    }

    #[test]
    fn task_ordering_nested_paths() {
        let a = task(&["git", "clone"], None);
        let b = task(&["git", "push"], None);
        assert!(a < b);
    }

    #[test]
    fn task_ordering_shorter_path_first() {
        let a = task(&["git"], None);
        let b = task(&["git", "clone"], None);
        assert!(a < b);
    }

    #[test]
    fn task_ordering_equal() {
        let a = task(&["same"], None);
        let b = task(&["same"], None);
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }

    #[test]
    fn task_equality_ignores_description() {
        let a = task(&["x"], Some("desc a"));
        let b = task(&["x"], Some("desc b"));
        // Ord considers only path
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }

    // --- TaskCollection ---

    #[test]
    fn collection_insert_sorted_maintains_order() {
        let mut col = TaskCollection::new();
        col.insert_task_sorted(task(&["c"], None));
        col.insert_task_sorted(task(&["a"], None));
        col.insert_task_sorted(task(&["b"], None));

        let names: Vec<&str> = col.tasks().iter()
            .map(|t| t.path[0].as_str())
            .collect();
        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn collection_insert_sorted_with_nested_paths() {
        let mut col = TaskCollection::new();
        col.insert_task_sorted(task(&["git", "push"], None));
        col.insert_task_sorted(task(&["docker", "build"], None));
        col.insert_task_sorted(task(&["git", "clone"], None));

        let paths: Vec<String> = col.tasks().iter()
            .map(|t| t.path.join("."))
            .collect();
        assert_eq!(paths, vec!["docker.build", "git.clone", "git.push"]);
    }

    #[test]
    fn collection_from_tasks() {
        let tasks = vec![task(&["b"], None), task(&["a"], None)];
        let col = TaskCollection::from_tasks(tasks);
        // from_tasks does NOT sort — preserves insertion order
        assert_eq!(col.tasks()[0].path[0], "b");
        assert_eq!(col.tasks()[1].path[0], "a");
    }

    #[test]
    fn collection_len_and_is_empty() {
        let mut col = TaskCollection::new();
        assert!(col.is_empty());
        assert_eq!(col.len(), 0);

        col.insert_task_sorted(task(&["x"], None));
        assert!(!col.is_empty());
        assert_eq!(col.len(), 1);
    }

    #[test]
    fn collection_into_tasks() {
        let mut col = TaskCollection::new();
        col.insert_task_sorted(task(&["a"], None));
        col.insert_task_sorted(task(&["b"], None));

        let tasks = col.into_tasks();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].path[0], "a");
    }

    // --- Visibility filtering ---

    #[test]
    fn visible_tasks_excludes_hidden() {
        let mut col = TaskCollection::new();
        col.insert_task_sorted(task(&["visible"], None));
        col.insert_task_sorted(hidden_task(&["hidden"]));
        col.insert_task_sorted(task(&["also_visible"], None));

        let visible: Vec<&str> = col.get_visible_tasks()
            .map(|t| t.path[0].as_str())
            .collect();
        assert_eq!(visible, vec!["also_visible", "visible"]);
    }

    #[test]
    fn visible_task_count() {
        let mut col = TaskCollection::new();
        col.insert_task_sorted(task(&["a"], None));
        col.insert_task_sorted(hidden_task(&["b"]));
        col.insert_task_sorted(task(&["c"], None));

        assert_eq!(col.get_number_of_visible_tasks(), 2);
    }

    #[test]
    fn all_hidden_returns_zero_visible() {
        let mut col = TaskCollection::new();
        col.insert_task_sorted(hidden_task(&["a"]));
        col.insert_task_sorted(hidden_task(&["b"]));

        assert_eq!(col.get_number_of_visible_tasks(), 0);
        assert_eq!(col.get_visible_tasks().count(), 0);
    }
}
