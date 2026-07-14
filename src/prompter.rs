use crate::parser::TerraformVariable;
use inquire::Text;
use std::collections::HashMap;

pub fn prompt_for_variables(variables: Vec<TerraformVariable>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for variable in variables {
        // 1. Initialize a text prompt for the variable
        let mut prompt = Text::new(&variable.name);
        // 2. If the variable has description, add it as help text
        if let Some(desc) = &variable.description {
            prompt = prompt.with_help_message(desc);
        }
        // 3. If the variable has a default value, set it as the default for the prompt
        if let Some(default_val) = &variable.default_value {
            prompt = prompt.with_default(default_val);
        }
        // 4. Prompt the user and handle the Result
        match prompt.prompt() {
            Ok(value) => {
                map.insert(variable.name, value);
            }
            Err(e) => {
                // Handle cancellation (Ctrl+C) gracefully by exiting the program
                eprintln!("Prompt cancelled: {}", e);
                std::process::exit(1);
            }
        }
    }
    map
}
