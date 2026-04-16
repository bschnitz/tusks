use unicode_width::UnicodeWidthStr;
use owo_colors::OwoColorize;
use crate::list::models::{List, ListGroup, ListGroupHeader, ListTask, RenderConfig};

macro_rules! colored {
    ($text:expr, $styled:expr, $config:expr) => {
        if $config.use_colors {
            format!("{}", $styled)
        } else {
            $text.to_string()
        }
    };
}

impl ListGroupHeader {
    fn print(&self, config: &RenderConfig) {
        if let Some(header) = &self.name {
            let indent = " ".repeat(config.header_indent);
            let styled = colored!(header, header.bright_blue().bold(), config);
            println!("{}{}\n", indent, styled);
        }
    }
}

impl ListTask {
    fn print(&self, config: &RenderConfig, align_col: usize) {
        let indent = " ".repeat(config.task_indent);
        let name_width = UnicodeWidthStr::width(self.name.as_str());
        
        match &self.description {
            Some(desc) => {
                let current_pos = config.task_indent + name_width;
                let gap_size = align_col.saturating_sub(current_pos);
                let gap = config.span_token.to_string().repeat(gap_size);
                
                let name_styled = colored!(&self.name, self.name.green(), config);
                let gap_styled = colored!(&gap, gap.bright_black(), config);
                let desc_styled = colored!(desc, desc.yellow(), config);
                
                println!("{}{} {} {}", indent, name_styled, gap_styled, desc_styled);
            }
            None => {
                let name_styled = colored!(&self.name, self.name.green(), config);
                println!("{}{}", indent, name_styled);
            }
        }
    }
}

impl ListGroup {
    fn print(&self, config: &RenderConfig, align_col: usize) {
        self.header.print(config);
        for task in &self.tasks {
            task.print(config, align_col);
        }
    }
}

impl List {
    fn calculate_align_column(&self, config: &RenderConfig) -> usize {
        let max_task_width = self.groups.iter()
            .flat_map(|group| &group.tasks)
            .map(|task| UnicodeWidthStr::width(task.name.as_str()))
            .max()
            .unwrap_or(0);
        
        config.task_indent + max_task_width + config.min_gap
    }
    
    pub fn print(&self, config: &RenderConfig) {
        eprintln!("List {:?}", self);
        if let Some(description) = &self.description {
            let styled = colored!(description, description.cyan().bold(), config);
            println!("{}\n", styled);
        }
        
        let align_col = self.calculate_align_column(config);
        
        for (i, group) in self.groups.iter().enumerate() {
            if i > 0 {
                println!();
            }
            group.print(config, align_col);
        }
    }
}
