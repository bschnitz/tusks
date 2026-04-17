use crate::task_list::models::{TaskCollection, TaskGroup, TaskList};
use crate::list::models::{List, ListGroup, ListGroupHeader, ListTask};

impl TaskGroup {
    fn to_list_groups(self, separator: &str) -> Vec<ListGroup> {
        self.to_list_groups_recursive(separator, vec![])
    }

    fn to_list_groups_recursive(self, separator: &str, path: Vec<&str>) -> Vec<ListGroup> {
        let direct_tasks_group = tasks_to_list_group(
            self.direct_tasks,
            separator,
            if path.is_empty() { None } else { Some(path.join(separator)) }
        );

        let subgroup_groups = self.subgroups.into_iter().flat_map(|(k, group)| {
            let mut new_path = path.clone();
            new_path.push(k.as_str());
            group.to_list_groups_recursive(separator, new_path)
        });

        std::iter::once(direct_tasks_group)
            .chain(subgroup_groups.into_iter())
            .collect()
    }
}

fn tasks_to_list_group(
    tasks: TaskCollection,
    separator: &str,
    groupname: Option<String>
) -> ListGroup
{
    let tasks = tasks.get_visible_tasks().map(|task| ListTask {
        name: task.path.join(separator),
        description: task.description.clone()
    }).collect();
    let header = ListGroupHeader { name: groupname };
    ListGroup { header, tasks }
}

impl TaskList {
    pub fn to_list(self) -> List {
        List {
            description: self.description.clone(),
            groups: self.root.to_list_groups(&self.separator)
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Command;
    use crate::task_list::models::TaskList;

    #[test]
    fn to_list_preserves_description() {
        let cmd = Command::new("root")
            .about("My CLI tool")
            .subcommand(Command::new("task1"));

        let list = TaskList::from_command(&cmd, ".".into(), 5, 20).to_list();
        assert_eq!(list.description.as_deref(), Some("My CLI tool"));
    }

    #[test]
    fn to_list_no_description() {
        let cmd = Command::new("root")
            .subcommand(Command::new("task1"));

        let list = TaskList::from_command(&cmd, ".".into(), 5, 20).to_list();
        assert!(list.description.is_none());
    }

    #[test]
    fn to_list_flat_tasks_use_separator() {
        let cmd = Command::new("root")
            .subcommand(
                Command::new("git")
                    .subcommand(Command::new("clone"))
                    .subcommand(Command::new("push"))
            );

        let list = TaskList::from_command(&cmd, "::".into(), 5, 20).to_list();
        let task_names: Vec<&str> = list.groups.iter()
            .flat_map(|g| g.tasks.iter())
            .map(|t| t.name.as_str())
            .collect();
        assert!(task_names.contains(&"git::clone"));
        assert!(task_names.contains(&"git::push"));
    }

    #[test]
    fn to_list_hidden_tasks_excluded() {
        let cmd = Command::new("root")
            .subcommand(Command::new("visible").about("yes"))
            .subcommand(Command::new("hidden").hide(true));

        let list = TaskList::from_command(&cmd, ".".into(), 5, 20).to_list();
        let task_names: Vec<&str> = list.groups.iter()
            .flat_map(|g| g.tasks.iter())
            .map(|t| t.name.as_str())
            .collect();
        assert!(task_names.contains(&"visible"));
        assert!(!task_names.contains(&"hidden"));
    }

    #[test]
    fn to_list_grouped_creates_named_groups() {
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

        // max_groupsize=2 forces subgroups
        let list = TaskList::from_command(&cmd, ".".into(), 2, 20).to_list();
        let group_names: Vec<Option<&str>> = list.groups.iter()
            .map(|g| g.header.name.as_deref())
            .collect();
        // Should have named groups for docker and git
        assert!(group_names.iter().any(|n| *n == Some("docker")));
        assert!(group_names.iter().any(|n| *n == Some("git")));
    }

    #[test]
    fn to_list_root_group_has_no_name() {
        let cmd = Command::new("root")
            .subcommand(Command::new("a"))
            .subcommand(Command::new("b"));

        let list = TaskList::from_command(&cmd, ".".into(), 5, 20).to_list();
        // The root group should have no header name
        assert!(list.groups[0].header.name.is_none());
    }

    #[test]
    fn to_list_task_descriptions_preserved() {
        let cmd = Command::new("root")
            .subcommand(Command::new("greet").about("Say hello"))
            .subcommand(Command::new("bye"));

        let list = TaskList::from_command(&cmd, ".".into(), 5, 20).to_list();
        let tasks: Vec<_> = list.groups.iter()
            .flat_map(|g| g.tasks.iter())
            .collect();

        let greet = tasks.iter().find(|t| t.name == "greet").unwrap();
        assert_eq!(greet.description.as_deref(), Some("Say hello"));

        let bye = tasks.iter().find(|t| t.name == "bye").unwrap();
        assert!(bye.description.is_none());
    }
}
