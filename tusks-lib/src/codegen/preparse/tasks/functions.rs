use syn::{Item, ItemFn, ItemMod, parse_quote};
use syn::{Attribute, Meta};

use crate::{AttributeValue, attribute::models::TasksConfig};

pub fn add_use_staements(module: &mut ItemMod) {
    let use_statement: Item = parse_quote! {
        use ::tusks::clap::{CommandFactory, Parser};
    };
    
    if let Some((_, ref mut items)) = module.content {
        items.insert(0, use_statement);
    }
}

pub fn set_allow_external_subcommands(module: &mut ItemMod) {
    if module.get_attribute_bool("command", "allow_external_subcommands") {
        return;
    }
    
    // Suche nach existierendem #[command(...)]-Attribut
    if let Some(attr) = module.attrs.iter_mut().find(|a| a.path().is_ident("command")) {
        // Vorhandenes command-Attribut gefunden
        match &mut attr.meta {
            Meta::List(list) => {
                // Prüfe, ob tokens mit Komma enden
                let tokens_str = list.tokens.to_string();
                let has_trailing_comma = tokens_str.trim_end().ends_with(',');
                
                let tokens = &list.tokens;
                if tokens.is_empty() {
                    list.tokens = parse_quote! { allow_external_subcommands = true };
                } else if has_trailing_comma {
                    // Komma ist bereits da, kein zusätzliches einfügen
                    list.tokens = parse_quote! { #tokens allow_external_subcommands = true };
                } else {
                    // Komma hinzufügen
                    list.tokens = parse_quote! { #tokens, allow_external_subcommands = true };
                }
            }
            Meta::Path(_) => {
                // #[command] ohne Argumente -> konvertiere zu #[command(allow_external_subcommands = true)]
                *attr = parse_quote! { #[command(allow_external_subcommands = true)] };
            }
            Meta::NameValue(_) => {
                // Unerwarteter Fall, ersetze komplett
                *attr = parse_quote! { #[command(allow_external_subcommands = true)] };
            }
        }
    } else {
        // Kein command-Attribut vorhanden -> füge neues hinzu
        let new_attr: Attribute = parse_quote! { #[command(allow_external_subcommands = true)] };
        module.attrs.push(new_attr);
    }
}

pub fn add_execute_task_function(module: &mut ItemMod, config: &TasksConfig) {
    let separator = &config.separator;
    let max_groupsize = &config.max_groupsize;
    let max_depth = &config.max_depth;
    let use_colors = &config.use_colors;
    let function: ItemFn = parse_quote! {
        #[command(about = "Execute a task", hide=true)]
        #[default]
        pub fn _execute_task(external_args: Vec<String>) -> Option<u8> {
            let command = __internal_tusks_module::cli::Cli::command();
            if let Some(first) = external_args.first() {
                let mut transformed_arguments = vec![command.get_name().to_string()];
                transformed_arguments.extend(first.split(#separator).map(|s| s.to_string()));
                transformed_arguments.extend_from_slice(&external_args[1..]);
                let cli = __internal_tusks_module::cli::Cli::parse_from(transformed_arguments);
                return __internal_tusks_module::handle_matches(&cli);
            }
            
            let task_list = ::tusks::tasks::task_list::models::TaskList::from_command(
                &command,
                #separator.to_string(),
                #max_groupsize,
                #max_depth
            );
            let mut render_config = ::tusks::tasks::list::models::RenderConfig::default();
            render_config.use_colors = #use_colors;
            task_list.to_list().print(&render_config);
            Some(0)
        }
    };
    
    if let Some((_, ref mut items)) = module.content {
        items.push(Item::Fn(function));
    }
}

pub fn add_show_help_for_task(module: &mut ItemMod, config: &TasksConfig) {
    let separator = &config.separator;
    let max_groupsize = &config.max_groupsize;
    let max_depth = &config.max_depth;

    let function: ItemFn = parse_quote! {
        #[command(about = "Show the help for a task", name="h", hide=true)]
        pub fn _show_help_for_task(#[arg()] task: Option<String>) {
            if let Some(task) = task {
                let command = __internal_tusks_module::cli::Cli::command();
                let parts: Vec<&str> = task.split(#separator).collect();

                let args: Vec<&str> = std::iter::once(command.get_name())
                    .chain(parts.iter().copied())
                    .chain(std::iter::once("--help"))
                    .collect();

                let cli = __internal_tusks_module::cli::Cli::parse_from(args);
                __internal_tusks_module::handle_matches(&cli);
            }
            else {
                let command = __internal_tusks_module::cli::Cli::command();
                let task_list = ::tusks::tasks::task_list::models::TaskList::from_command(
                    &command,
                    #separator.to_string(),
                    #max_groupsize,
                    #max_depth
                );
                task_list.to_list().print(&::tusks::tasks::list::models::RenderConfig::default());
            }
        }
    };

    if let Some((_, ref mut items)) = module.content {
        items.push(Item::Fn(function));
    }
}
