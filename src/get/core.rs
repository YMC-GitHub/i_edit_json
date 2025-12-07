//! Core functionality for get command

use anyhow::{Context, Result};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

use super::types::{ExtractConfig, ExtractionResult};
use super::utils::{format_output, get_nested_value, strip_quotes_internal};
use crate::error::JsonExtractError;

/// Extract a single field from a JSON file
pub fn extract_field(config: &ExtractConfig) -> Result<String> {
    let content = fs::read_to_string(&config.file_path)
        .context(format!("Failed to read file: {}", config.file_path))?;

    let value: JsonValue = serde_json::from_str(&content)
        .context(format!("Invalid JSON syntax in: {}", config.file_path))?;

    let field_value = get_nested_value(&value, &config.field_path)
        .context(format!("Field not found: {}", config.field_path))?;

    let mut result = format_output(field_value, config.output_format.as_deref())?;

    if config.strip_quotes {
        result = strip_quotes_internal(&result);
    }

    Ok(result)
}

/// Extract multiple fields from a JSON file
pub fn extract_multiple_fields(
    file_path: &str,
    field_paths: &[String],
    strip_quotes: bool,
) -> Result<ExtractionResult> {
    let content =
        fs::read_to_string(file_path).context(format!("Failed to read file: {}", file_path))?;

    let value: JsonValue =
        serde_json::from_str(&content).context(format!("Invalid JSON syntax in: {}", file_path))?;

    let mut result = ExtractionResult::new(file_path.to_string());

    for field_path in field_paths {
        let field_value = get_nested_value(&value, field_path)
            .context(format!("Field not found: {}", field_path))?;

        let mut formatted_value = format_output(field_value, None)?;
        if strip_quotes {
            formatted_value = strip_quotes_internal(&formatted_value);
        }

        result.add_field(field_path.clone(), formatted_value);
    }

    Ok(result)
}

/// Extract an array from a JSON file
pub fn extract_array(
    file_path: &str,
    array_path: &str,
    output_format: Option<&str>,
) -> Result<String> {
    let config = ExtractConfig {
        file_path: file_path.to_string(),
        field_path: array_path.to_string(),
        output_format: output_format.map(|s| s.to_string()),
        strip_quotes: false,
    };
    extract_field(&config)
}

/// Extract array length from a JSON file
pub fn extract_array_length(file_path: &str, array_path: &str) -> Result<usize> {
    let content =
        fs::read_to_string(file_path).context(format!("Failed to read file: {}", file_path))?;

    let value: JsonValue =
        serde_json::from_str(&content).context(format!("Invalid JSON syntax in: {}", file_path))?;

    let array_value =
        get_nested_value(&value, array_path).context(format!("Array not found: {}", array_path))?;

    let array = array_value
        .as_array()
        .ok_or_else(|| JsonExtractError::NotAnArray(array_path.to_string()))?;

    Ok(array.len())
}

/// Extract a specific array element from a JSON file
pub fn extract_array_element(
    file_path: &str,
    array_path: &str,
    index: usize,
    strip_quotes: bool,
) -> Result<String> {
    let content =
        fs::read_to_string(file_path).context(format!("Failed to read file: {}", file_path))?;

    let value: JsonValue =
        serde_json::from_str(&content).context(format!("Invalid JSON syntax in: {}", file_path))?;

    let array_value =
        get_nested_value(&value, array_path).context(format!("Array not found: {}", array_path))?;

    let array = array_value
        .as_array()
        .ok_or_else(|| JsonExtractError::NotAnArray(array_path.to_string()))?;

    if index >= array.len() {
        return Err(JsonExtractError::ArrayIndexOutOfBounds {
            path: array_path.to_string(),
            index,
            length: array.len(),
        }
        .into());
    }

    let element = &array[index];
    let mut result = format_output(element, None)?;

    if strip_quotes {
        result = strip_quotes_internal(&result);
    }

    Ok(result)
}

// Preset extraction functions for common JSON structures

/// Get the name from a package.json file
pub fn get_package_name(file_path: Option<&str>) -> Result<String> {
    let path = file_path.unwrap_or("package.json");
    let config = ExtractConfig {
        file_path: path.to_string(),
        field_path: "name".to_string(),
        output_format: None,
        strip_quotes: true,
    };
    extract_field(&config)
}

/// Get the version from a package.json file
pub fn get_package_version(file_path: Option<&str>) -> Result<String> {
    let path = file_path.unwrap_or("package.json");
    let config = ExtractConfig {
        file_path: path.to_string(),
        field_path: "version".to_string(),
        output_format: None,
        strip_quotes: true,
    };
    extract_field(&config)
}

/// Get all dependencies from a package.json file
pub fn get_dependencies(file_path: Option<&str>) -> Result<HashMap<String, String>> {
    let path = file_path.unwrap_or("package.json");
    let content = fs::read_to_string(path).context("Failed to read package.json")?;

    let value: JsonValue = serde_json::from_str(&content).context("Invalid JSON syntax")?;

    let mut dependencies = HashMap::new();

    if let Some(deps) = value.get("dependencies") {
        if let Some(obj) = deps.as_object() {
            for (name, value) in obj {
                dependencies.insert(name.clone(), value.as_str().unwrap_or("").to_string());
            }
        }
    }

    Ok(dependencies)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_field_basic() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"name": "test"}}"#).unwrap();
        let path = temp_file.path().to_str().unwrap();

        let config = ExtractConfig {
            file_path: path.to_string(),
            field_path: "name".to_string(),
            ..Default::default()
        };
        assert_eq!(extract_field(&config).unwrap(), "\"test\"");
    }

    #[test]
    fn test_extract_array_length() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"authors": ["Alice", "Bob"]}}"#).unwrap();
        let path = temp_file.path().to_str().unwrap();

        assert_eq!(extract_array_length(path, "authors").unwrap(), 2);
    }
}
