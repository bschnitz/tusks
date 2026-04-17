use indexmap::IndexMap;
use clap::Command;
use crate::task_list::models::{Task, TaskCollection, TaskGroup, TaskList};

impl TaskList {
    pub fn from_command(command: &Command, separator: String, max_groupsize: usize, max_depth: usize) -> Self {
        let task_collection = TaskCollection::from_command(command, vec![]);
        let root_group = TaskGroup::create_grouping(
            task_collection,
            0,
            max_depth,
            max_groupsize
        );

        TaskList {
            description: command.get_long_about().or(command.get_about()).map(|d| d.to_string()),
            max_groupsize,
            max_depth,
            separator,
            root: root_group
        }
    }
}

impl TaskGroup {
    fn create_grouping(
        tasks: TaskCollection,
        depth: usize,
        max_depth: usize,
        max_groupsize: usize,
    ) -> Self {
        // Base case: no further grouping possible or needed
        // Only count visible tasks for the grouping decision
        let visible_count = tasks.get_number_of_visible_tasks();
        
        if max_depth == 0 || visible_count <= max_groupsize {
            return TaskGroup {
                direct_tasks: tasks,
                subgroups: IndexMap::new(),
            };
        }

        // Group tasks by their path component at the current depth
        let groups = tasks.tasks.into_iter().fold(
            IndexMap::<String, TaskCollection>::new(),
            |mut groups, task| {
                groups.entry(task.path[depth].clone()).or_default().tasks.push(task);
                groups
            },
        );

        let mut subgroups = IndexMap::new();
        for (key, group_tasks) in groups {
            let subgroup = Self::create_grouping(
                group_tasks,
                depth + 1,
                max_depth - 1,
                max_groupsize,
            );
            subgroups.insert(key, subgroup);
        }

        let mut direct_tasks = Vec::new();
        subgroups.retain(|_key, subgroup| {
            if subgroup.direct_tasks.get_number_of_visible_tasks() <= 1
               && subgroup.subgroups.is_empty()
            {
                direct_tasks.extend(subgroup.direct_tasks.tasks.clone());
                false
            } else {
                true
            }
        });

        TaskGroup {
            direct_tasks: TaskCollection {tasks: direct_tasks},
            subgroups,
        }
    }
}

impl TaskCollection {
    fn from_command(command: &Command, current_path: Vec<String>) -> Self {
        let mut collection = TaskCollection::new();
        collection.extract_tasks_recursive(command, current_path);
        collection
    }

    fn extract_tasks_recursive(&mut self, command: &Command, current_path: Vec<String>) {
        // Check if this command has subcommands (it's a parent command)
        if command.get_subcommands().count() == 0 {
            let about = command.get_about().map(|s| s.to_string());
            let task = Task {
                path: current_path,
                description: about,
                hidden: command.is_hide_set()
            };
            self.insert_task_sorted(task);
        } else {
            // This command has subcommands, recurse into them
            for subcommand in command.get_subcommands() {
                let mut new_path = current_path.clone();
                new_path.push(subcommand.get_name().to_string());
                self.extract_tasks_recursive(subcommand, new_path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- TaskCollection::from_command ---

    #[test]
    fn from_command_single_leaf() {
        let cmd = Command::new("root")
            .subcommand(Command::new("hello").about("Say hello"));

        let col = TaskCollection::from_command(&cmd, vec![]);
        assert_eq!(col.len(), 1);
        assert_eq!(col.tasks()[0].path, vec!["hello"]);
        assert_eq!(col.tasks()[0].description.as_deref(), Some("Say hello"));
    }

    #[test]
    fn from_command_nested_subcommands() {
        let cmd = Command::new("root")
            .subcommand(
                Command::new("git")
                    .subcommand(Command::new("clone"))
                    .subcommand(Command::new("push"))
            )
            .subcommand(Command::new("build"));

        let col = TaskCollection::from_command(&cmd, vec![]);
        let paths: Vec<String> = col.tasks().iter()
            .map(|t| t.path.join("."))
            .collect();
        assert_eq!(paths, vec!["build", "git.clone", "git.push"]);
    }

    #[test]
    fn from_command_preserves_hidden_flag() {
        let cmd = Command::new("root")
            .subcommand(Command::new("visible"))
            .subcommand(Command::new("hidden").hide(true));

        let col = TaskCollection::from_command(&cmd, vec![]);
        assert_eq!(col.len(), 2);

        let visible = col.tasks().iter().find(|t| t.path[0] == "visible").unwrap();
        assert!(!visible.hidden);

        let hidden = col.tasks().iter().find(|t| t.path[0] == "hidden").unwrap();
        assert!(hidden.hidden);
    }

    #[test]
    fn from_command_deeply_nested() {
        let cmd = Command::new("root")
            .subcommand(
                Command::new("a")
                    .subcommand(
                        Command::new("b")
                            .subcommand(Command::new("c"))
                    )
            );

        let col = TaskCollection::from_command(&cmd, vec![]);
        assert_eq!(col.len(), 1);
        assert_eq!(col.tasks()[0].path, vec!["a", "b", "c"]);
    }

    #[test]
    fn from_command_no_subcommands_yields_single_root_task() {
        let cmd = Command::new("root");
        let col = TaskCollection::from_command(&cmd, vec![]);
        assert_eq!(col.len(), 1);
        assert!(col.tasks()[0].path.is_empty());
    }

    // --- TaskGroup::create_grouping ---

    #[test]
    fn grouping_small_set_stays_flat() {
        let cmd = Command::new("root")
            .subcommand(Command::new("a"))
            .subcommand(Command::new("b"))
            .subcommand(Command::new("c"));

        let task_list = TaskList::from_command(&cmd, ".".into(), 5, 20);
        assert!(task_list.root.subgroups.is_empty());
        assert_eq!(task_list.root.direct_tasks.len(), 3);
    }

    #[test]
    fn grouping_creates_subgroups_when_exceeding_max() {
        let cmd = Command::new("root")
            .subcommand(
                Command::new("git")
                    .subcommand(Command::new("clone"))
                    .subcommand(Command::new("push"))
                    .subcommand(Command::new("pull"))
            )
            .subcommand(
                Command::new("docker")
                    .subcommand(Command::new("build"))
                    .subcommand(Command::new("run"))
                    .subcommand(Command::new("stop"))
            );

        let task_list = TaskList::from_command(&cmd, ".".into(), 2, 20);
        assert!(!task_list.root.subgroups.is_empty());
    }

    #[test]
    fn grouping_collapses_single_task_subgroups() {
        let cmd = Command::new("root")
            .subcommand(
                Command::new("git")
                    .subcommand(Command::new("clone"))
                    .subcommand(Command::new("push"))
                    .subcommand(Command::new("pull"))
            )
            .subcommand(
                Command::new("docker")
                    .subcommand(Command::new("build"))
            );

        let task_list = TaskList::from_command(&cmd, ".".into(), 1, 20);
        // docker has only 1 task → collapsed to direct_tasks
        assert!(!task_list.root.subgroups.contains_key("docker"));
        // git has 3 tasks → remains as subgroup
        assert!(task_list.root.subgroups.contains_key("git"));
    }

    #[test]
    fn grouping_max_depth_zero_stays_flat() {
        let cmd = Command::new("root")
            .subcommand(
                Command::new("git")
                    .subcommand(Command::new("clone"))
                    .subcommand(Command::new("push"))
            );

        let task_list = TaskList::from_command(&cmd, ".".into(), 1, 0);
        assert!(task_list.root.subgroups.is_empty());
        assert_eq!(task_list.root.direct_tasks.len(), 2);
    }

    #[test]
    fn grouping_hidden_tasks_not_counted_for_threshold() {
        let cmd = Command::new("root")
            .subcommand(
                Command::new("git")
                    .subcommand(Command::new("clone"))
                    .subcommand(Command::new("push"))
            )
            .subcommand(Command::new("hidden1").hide(true))
            .subcommand(Command::new("hidden2").hide(true))
            .subcommand(Command::new("hidden3").hide(true));

        // 5 total but only 2 visible, max_groupsize=3
        let task_list = TaskList::from_command(&cmd, ".".into(), 3, 20);
        assert!(task_list.root.subgroups.is_empty());
    }

    // --- TaskList::from_command metadata ---

    #[test]
    fn from_command_captures_description() {
        let cmd = Command::new("root")
            .about("Short desc")
            .subcommand(Command::new("task1"));

        let task_list = TaskList::from_command(&cmd, ".".into(), 5, 20);
        assert_eq!(task_list.description.as_deref(), Some("Short desc"));
    }

    #[test]
    fn from_command_prefers_long_about() {
        let cmd = Command::new("root")
            .about("Short")
            .long_about("Long description")
            .subcommand(Command::new("task1"));

        let task_list = TaskList::from_command(&cmd, ".".into(), 5, 20);
        assert_eq!(task_list.description.as_deref(), Some("Long description"));
    }

    #[test]
    fn from_command_stores_config() {
        let task_list = TaskList::from_command(
            &Command::new("root").subcommand(Command::new("x")),
            "::".into(), 10, 3
        );
        assert_eq!(task_list.separator, "::");
        assert_eq!(task_list.max_groupsize, 10);
        assert_eq!(task_list.max_depth, 3);
    }
}
