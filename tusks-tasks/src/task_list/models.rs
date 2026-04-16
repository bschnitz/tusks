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
