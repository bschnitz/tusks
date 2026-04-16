# Funktionenanalyse aller Rust-Dateien

Hier ist die vollständige Liste aller Funktionen mit ihren Namen und Längen pro Datei:

## 1. src/codegen/handle_matches/arms/submodule.rs (236 Zeilen)
- `build_submodule_match_arm`: 17 Zeilen
- `build_variant_ident`: 3 Zeilen
- `build_parameter_pattern_bindings`: 18 Zeilen
- `has_commands`: 5 Zeilen
- `add_sub_field_if_needed`: 7 Zeilen
- `build_parameter_initialization`: 50 Zeilen
- `build_nested_match_arms`: 29 Zeilen
- `build_final_match_arm`: 15 Zeilen

## 2. src/codegen/handle_matches/module.rs (184 Zeilen)
- `build_handle_matches`: 28 Zeilen
- `build_parameters_initialization`: 26 Zeilen
- `build_match_arms_recursive`: 50 Zeilen
- `build_no_command_error_arm`: 17 Zeilen
- `build_external_arm`: 36 Zeilen
- `tusk_has_parameters_arg`: 8 Zeilen

## 3. src/codegen/handle_matches/arms/function.rs (254 Zeilen)
- `build_function_match_arm`: 23 Zeilen
- `build_default_function_match_arm`: 21 Zeilen
- `build_external_subcommand_match_arm`: 22 Zeilen
- `build_function_call`: 35 Zeilen
- `build_pattern_bindings`: 23 Zeilen
- `build_pattern_fields`: 14 Zeilen
- `build_function_arguments`: 43 Zeilen
- `build_function_path`: 19 Zeilen

## 4. src/parsing/tusk.rs (273 Zeilen)
- `from_fn`: 24 Zeilen
- `validate_return_type`: 19 Zeilen
- `is_u8_type`: 11 Zeilen
- `is_option_u8_type`: 27 Zeilen
- `validate`: 8 Zeilen
- `check_duplicate_default`: 11 Zeilen
- `validate_default_function_arguments`: 18 Zeilen
- `validate_single_argument`: 24 Zeilen
- `validate_two_arguments`: 32 Zeilen
- `is_parameters_reference`: 15 Zeilen
- `error_single_argument`: 15 Zeilen
- `error_two_arguments_signature`: 7 Zeilen
- `error_message_too_many_args`: 9 Zeilen
- `is_vec_string`: 23 Zeilen

## 5. src/codegen/cli/module.rs (384 Zeilen)
- `build_cli`: 40 Zeilen
- `build_cli_struct`: 36 Zeilen
- `build_cli_fields_from_parameters`: 33 Zeilen
- `dereference_type`: 8 Zeilen
- `build_external_commands_enum`: 41 Zeilen
- `build_commands_enum`: 42 Zeilen
- `build_command_variant_from_tusk`: 18 Zeilen
- `build_fields_from_tusk_params`: 58 Zeilen
- `is_parameters_type`: 11 Zeilen
- `build_command_variant_from_submodule`: 34 Zeilen
- `build_enum_fields_from_parameters`: 33 Zeilen

## 6. src/lib.rs (10 Zeilen)
- Keine Funktionen

## 7. src/codegen/preparse/tasks/functions.rs (141 Zeilen)
- `add_use_staements`: 6 Zeilen
- `set_allow_external_subcommands`: 39 Zeilen
- `add_execute_task_function`: 40 Zeilen
- `add_show_help_for_task`: 37 Zeilen

## 8. src/parsing/module.rs (189 Zeilen)
- `from_module`: 50 Zeilen
- `validate_is_root_or_has_parent`: 27 Zeilen
- `extract_module_items`: 39 Zeilen
- `parse_struct`: 6 Zeilen
- `extract_external_modules`: 49 Zeilen

## 9. src/codegen/mod.rs (6 Zeilen)
- Keine Funktionen

## 10. src/codegen/preparse/mod.rs (2 Zeilen)
- Keine Funktionen

## 11. src/codegen/preparse/tasks/mod.rs (2 Zeilen)
- Keine Funktionen

## 12. src/parsing/attribute/mod.rs (3 Zeilen)
- Keine Funktionen

## 13. src/parsing/mod.rs (6 Zeilen)
- Keine Funktionen

## 14. src/parsing/attribute/parse.rs (141 Zeilen)
- `parse` (TusksAttr): 19 Zeilen
- `parse` (TasksConfig): 18 Zeilen
- `parse_bool_flag`: 9 Zeilen
- `parse_nested_config`: 4 Zeilen
- `parse_trailing_comma`: 12 Zeilen
- `parse_usize`: 7 Zeilen
- `parse_string`: 4 Zeilen
- `unknown_attribute_error`: 6 Zeilen
- `unknown_parameter_error`: 7 Zeilen

## 15. src/parsing/attribute/models.rs (34 Zeilen)
- Keine Funktionen

## 16. src/parsing/util/get_attribute_value.rs (147 Zeilen)
- `get_attribute_value` (Trait): 17 Zeilen
- `get_attribute_bool` (Trait): 4 Zeilen
- `extract_value`: 16 Zeilen

## 17. src/parsing/util/mod.rs (3 Zeilen)
- Keine Funktionen

## 18. src/codegen/util/mod.rs (4 Zeilen)
- Keine Funktionen

## 19. src/models.rs (102 Zeilen)
- Keine Funktionen

## 20. src/codegen/parameters/module.rs (230 Zeilen)
- `supplement_parameters`: 55 Zeilen
- `extract_lifetime`: 13 Zeilen
- `add_parameters_struct`: 29 Zeilen
- `add_struct_to_module`: 11 Zeilen
- `find_parameters_struct_mut`: 41 Zeilen
- `add_super_field_to_parameters_struct`: 46 Zeilen
- `add_phantom_field_to_struct`: 45 Zeilen

## 21. src/codegen/util/command_attribute.rs (194 Zeilen)
- `generate_command_attribute`: 10 Zeilen
- `generate_command_attribute_for_subcommands`: 30 Zeilen
- `generate_command_attribute_for_external_subcommands`: 9 Zeilen
- `generate_command_attribute` (Tusk): 4 Zeilen
- `generate_command_attribute` (ExternalModule): 4 Zeilen
- `use_attributes_or_default`: 7 Zeilen
- `transform_attributes_to_command`: 60 Zeilen

## 22. src/codegen/util/attribute.rs (50 Zeilen)
- `extract_attributes` (TusksParameters): 12 Zeilen
- `extract_attributes` (Tusk): 6 Zeilen
- `extract_attributes` (ExternalModule): 8 Zeilen
- `extract_attributes` (TusksModule): 7 Zeilen

## 23. src/parsing/util/attr.rs (69 Zeilen)
- `has_attr` (Trait): 10 Zeilen
- `attrs` Implementierungen: 39 Zeilen

## 24. src/codegen/util/enum_util.rs (20 Zeilen)
- `convert_function_to_enum_variant`: 4 Zeilen
- `convert_submodule_to_enum_variant`: 4 Zeilen
- `convert_external_module_to_enum_variant`: 3 Zeilen

## 25. src/codegen/handle_matches/arms/mod.rs (4 Zeilen)
- Keine Funktionen

## 26. src/codegen/handle_matches/mod.rs (3 Zeilen)
- Keine Funktionen

## 27. src/codegen/handle_matches/arms/external_module.rs (1 Zeilen)
- Keine Funktionen

## 28. src/codegen/parameters/mod.rs (2 Zeilen)
- Keine Funktionen

## 29. src/parsing/parameters.rs (56 Zeilen)
- `from_struct`: 42 Zeilen
- `is_reference_type`: 3 Zeilen

## 30. src/codegen/cli/parameters.rs (1 Zeilen)
- Keine Funktionen

## 31. src/codegen/cli/tusk.rs (1 Zeilen)
- Keine Funktionen

## Gesamtstatistik
- **Dateien mit Code**: 24
- **Dateien ohne Code**: 7
- **Gesamtzeilen**: 2,842 Zeilen
- **GesamtFunktionen**: 92
- **Durchschnittliche Funktionslänge**: 15.7 Zeilen