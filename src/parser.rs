use hcl;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct TerraformVariable {
    pub name: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub var_type: Option<String>,
}

/// Scans the given directory for `.tf` files, parses them, and extracts variables.
pub fn discover_variables(dir_path: &Path) -> Vec<TerraformVariable> {
    let mut variables = Vec::new();

    // 1. Read the directory. If it fails (e.g., doesn't exist), return the empty Vec.
    let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => return variables,
    };

    // 2. Iterate through each entry in the directory.
    for entry in entries.flatten() {
        let file_path = entry.path();

        // 3. Check if the entry is a file and has a ".tf" extension.
        if file_path.is_file() && file_path.extension().and_then(|s| s.to_str()) == Some("tf") {
            // 4. Read the file contents into a String.
            if let Ok(content) = fs::read_to_string(&file_path) {
                // 5. Parse the HCL string into an AST (hcl::Body).
                if let Ok(parsed_body) = hcl::parse(&content) {
                    // 6. Extract the variables and append them to our main list.
                    let mut extracted = extract_variables(parsed_body);
                    variables.append(&mut extracted);
                } else {
                    eprintln!(
                        "Warning: Failed to parse HCL in file: {}",
                        file_path.display()
                    );
                }
            } else {
                eprintln!("Warning: Failed to read file: {}", file_path.display());
            }
        }
    }

    variables
}

/// Traverses the HCL AST and extracts data from `variable` blocks.
fn extract_variables(body: hcl::Body) -> Vec<TerraformVariable> {
    let mut vars = Vec::new();

    // Iterate through all top-level structures in the HCL document.
    for structure in body.into_iter() {
        // We only care about Blocks (e.g., `variable "foo" { ... }`)
        if let hcl::Structure::Block(block) = structure {
            // Check if the block's identifier is exactly "variable"
            if block.identifier.as_str() == "variable" {
                // The name of the variable is the first label.
                let name = block
                    .labels
                    .first()
                    .map(|l| l.as_str().to_string())
                    .unwrap_or_default();

                let mut description = None;
                let mut default_value = None;
                let mut var_type = None;

                // Iterate through the attributes inside the variable block.
                for attr_struct in block.body.into_iter() {
                    if let hcl::Structure::Attribute(attr) = attr_struct {
                        let key = attr.key.as_str();

                        // We format the expression back into an HCL string representation.
                        // We trim surrounding quotes because standard strings come out wrapped.
                        let val_str = hcl::format::to_string(&attr.expr)
                            .unwrap_or_default()
                            .trim_matches('"')
                            .to_string();

                        match key {
                            "description" => description = Some(val_str),
                            "default" => default_value = Some(val_str),
                            "type" => var_type = Some(val_str),
                            _ => {} // Ignore any other attributes like 'sensitive' or 'validation' for now
                        }
                    }
                }

                // Construct our struct and add it to the list
                vars.push(TerraformVariable {
                    name,
                    description,
                    default_value,
                    var_type,
                });
            }
        }
    }
    vars
}
