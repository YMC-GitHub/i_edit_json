use crate::{
    set::core::{set_field, set_field_and_save},
    SetConfig,
};
use anyhow::{Context, Result};
use clap::{Arg, Command};

/// Define set command CLI structure
pub fn cli() -> Command {
    Command::new("set")
        .about("Set values in JSON files")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("JSON file path")
                .default_value("package.json"),
        )
        .arg(
            Arg::new("field")
                .short('k')
                .long("field")
                .value_name("FIELD")
                .help("Dot-separated field path (e.g., name, dependencies.serde)")
                .required(true),
        )
        .arg(
            Arg::new("value")
                .short('v')
                .long("value")
                .value_name("VALUE")
                .help("Value to set for the field")
                .required(true),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Value type (string, integer, float, boolean, null, auto)")
                .default_value("auto"),
        )
        .arg(
            Arg::new("create-missing")
                .long("create-missing")
                .help("Create missing parent fields if they don't exist")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("in-place")
                .short('i')
                .long("in-place")
                .help("Modify the file in place")
                .action(clap::ArgAction::SetTrue),
        )
}

/// Handle set command logic
pub fn handle_set_command(matches: &clap::ArgMatches) -> Result<()> {
    let file_path = matches
        .get_one::<String>("file")
        .context("File path is required")?;
    let field_path = matches
        .get_one::<String>("field")
        .context("Field path is required")?;
    let value = matches
        .get_one::<String>("value")
        .context("Value is required")?;
    let value_type = matches
        .get_one::<String>("type")
        .context("Value type is required")?;
    let create_missing = matches.get_flag("create-missing");
    let in_place = matches.get_flag("in-place");

    // Handle value type (auto-detect or specified type)
    let value_type = if value_type == "auto" {
        None
    } else {
        Some(value_type.as_str())
    };

    // Build configuration
    let config = SetConfig {
        file_path: file_path.to_string(),
        field_path: field_path.to_string(),
        value: value.to_string(),
        value_type: value_type.map(|s| s.to_string()),
        create_missing,
    };

    // Execute set operation
    if in_place {
        // Modify file in place
        set_field_and_save(&config)?;
        println!(
            "✅ Field '{}' set to '{}' in {}",
            field_path, value, file_path
        );
    } else {
        // Output modified content (don't modify original file)
        let result = set_field(&config)?;
        println!("{}", result);
    }

    Ok(())
}
