//! Core implementation for setting JSON fields

use anyhow::{Context, Result};
use serde_json::{Map, Value as JsonValue};
use std::fs;

use super::types::SetConfig;
use super::utils::{parse_value_with_type, split_field_path};
use crate::error::JsonExtractError;

/// Set a field in JSON file and return updated content
pub fn set_field(config: &SetConfig) -> Result<String> {
    // Read file content
    let content = fs::read_to_string(&config.file_path)
        .with_context(|| format!("Failed to read file: {}", config.file_path))?;

    // Parse JSON
    let mut json_value: JsonValue =
        serde_json::from_str(&content).map_err(|e| JsonExtractError::InvalidJson {
            file: config.file_path.clone(),
            error: e.to_string(),
        })?;

    // Split field path
    let parts = split_field_path(&config.field_path)?;

    // Set nested value
    set_nested_value(
        &mut json_value,
        &parts,
        config.value.as_str(),
        config.value_type.as_deref(),
        config.create_missing,
    )?;

    // Convert back to JSON string
    let updated_content = serde_json::to_string_pretty(&json_value)?;
    Ok(updated_content)
}

/// Recursively set nested value in JSON structure
fn set_nested_value(
    current: &mut JsonValue,
    parts: &[String],
    value: &str,
    value_type: Option<&str>,
    create_missing: bool,
) -> Result<(), JsonExtractError> {
    if parts.is_empty() {
        return Err(JsonExtractError::FieldNotFound("Empty path".to_string()));
    }

    let (first, rest) = parts.split_first().unwrap();

    // Handle array syntax (e.g., "arr[0]")
    if first.contains('[') {
        let bracket_start = first.find('[').ok_or_else(|| {
            JsonExtractError::InvalidArrayIndex(format!("Invalid array syntax: {}", first))
        })?;
        let array_name = &first[..bracket_start];
        let index_part = &first[bracket_start + 1..first.len() - 1];
        let index: usize = index_part
            .parse()
            .map_err(|_| JsonExtractError::InvalidArrayIndex(index_part.to_string()))?;

        // Ensure parent is an object
        let current_obj = current.as_object_mut().ok_or_else(|| {
            JsonExtractError::NotAnObject(format!("Parent of {} is not an object", array_name))
        })?;

        // Get or create array
        let array = current_obj
            .entry(array_name)
            .or_insert_with(|| JsonValue::Array(Vec::new()))
            .as_array_mut()
            .ok_or_else(|| JsonExtractError::NotAnArray(array_name.to_string()))?;

        // Ensure array has enough elements if creating missing
        if create_missing {
            while array.len() <= index {
                array.push(JsonValue::Null);
            }
        }

        if rest.is_empty() {
            let array_len = array.len();

            // Set array element value
            let parsed_value = parse_value_with_type(value, value_type)?;
            array
                .get_mut(index)
                .ok_or_else(|| JsonExtractError::ArrayIndexOutOfBounds {
                    path: array_name.to_string(),
                    index,
                    length: array_len,
                })?
                .clone_from(&parsed_value);
        } else {
            let array_len = array.len();
            // Recurse into nested structure
            let elem =
                array
                    .get_mut(index)
                    .ok_or_else(|| JsonExtractError::ArrayIndexOutOfBounds {
                        path: array_name.to_string(),
                        index,
                        length: array_len,
                    })?;
            set_nested_value(elem, rest, value, value_type, create_missing)?;
        }
    } else {
        // Handle regular fields
        if rest.is_empty() {
            // Set final field value
            let parsed_value = parse_value_with_type(value, value_type)?;
            if let JsonValue::Object(obj) = current {
                obj.insert(first.clone(), parsed_value);
            } else if create_missing {
                // Create parent object if missing and allowed
                *current = JsonValue::Object(Map::new());
                current
                    .as_object_mut()
                    .unwrap()
                    .insert(first.clone(), parsed_value);
            } else {
                return Err(JsonExtractError::NotAnObject(format!(
                    "Cannot set field {} on non-object value",
                    first
                )));
            }
        } else {
            // Recurse into child fields
            let next = if let Some(obj) = current.as_object_mut() {
                obj.entry(first.clone())
                    .or_insert_with(|| JsonValue::Object(Map::new()))
            } else if create_missing {
                *current = JsonValue::Object(Map::new());
                current
                    .as_object_mut()
                    .unwrap()
                    .entry(first.clone())
                    .or_insert_with(|| JsonValue::Object(Map::new()))
            } else {
                return Err(JsonExtractError::NotAnObject(first.clone()));
            };

            set_nested_value(next, rest, value, value_type, create_missing)?;
        }
    }

    Ok(())
}

/// Set field and save changes to file
pub fn set_field_and_save(config: &SetConfig) -> Result<()> {
    let updated_content = set_field(config)?;
    fs::write(&config.file_path, updated_content)
        .with_context(|| format!("Failed to write to file: {}", config.file_path))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_set_field_basic() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"name": "old"}}"#).unwrap();
        let path = temp_file.path().to_str().unwrap();

        let config = SetConfig {
            file_path: path.to_string(),
            field_path: "name".to_string(),
            value: "new".to_string(),
            value_type: None,
            create_missing: false,
        };

        let updated = set_field(&config).unwrap();
        assert!(updated.contains(r#""name": "new""#));
    }

    #[test]
    fn test_set_array_element() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"authors": ["Alice", "Bob"]}}"#).unwrap();
        let path = temp_file.path().to_str().unwrap();

        let config = SetConfig {
            file_path: path.to_string(),
            field_path: "authors[0]".to_string(),
            value: "Charlie".to_string(),
            value_type: None,
            create_missing: false,
        };

        let updated = set_field(&config).unwrap();
        // println!("Updated: {}", updated); // Debug print to see the erro
        // assert!(updated.contains(r#""authors": ["Charlie", "Bob"]"#));

        // 方法1：使用更灵活的检查（检查是否包含 "Charlie" 和 "Bob"）
        assert!(updated.contains("Charlie"));
        assert!(updated.contains("Bob"));

        // 方法2：移除所有空白字符后检查
        let compact = updated.replace(char::is_whitespace, "");
        assert!(compact.contains(r#""authors":["Charlie","Bob"]"#));

        // 方法3：解析为 JSON 对象检查（最好的方法）
        let parsed: serde_json::Value = serde_json::from_str(&updated).unwrap();
        let authors = parsed["authors"].as_array().unwrap();
        assert_eq!(authors[0], "Charlie");
        assert_eq!(authors[1], "Bob");
    }
}
