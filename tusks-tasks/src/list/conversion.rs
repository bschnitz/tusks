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
