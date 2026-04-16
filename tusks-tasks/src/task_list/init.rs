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
        // Basisfall: keine weitere Gruppierung möglich oder nötig
        // Zähle nur sichtbare Tasks für die Gruppierungsentscheidung
        let visible_count = tasks.get_number_of_visible_tasks();
        
        if max_depth == 0 || visible_count <= max_groupsize {
            return TaskGroup {
                direct_tasks: tasks,
                subgroups: IndexMap::new(),
            };
        }

        // Rest bleibt gleich...
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
