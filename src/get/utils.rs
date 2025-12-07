//! Utility functions for get command

use anyhow::Result;
use serde_json::Value as JsonValue;

use crate::error::JsonExtractError;

/// Resolve nested value from JSON structure using dot-separated path with array support
pub fn get_nested_value<'a>(
    value: &'a JsonValue,
    path: &str,
) -> Result<&'a JsonValue, JsonExtractError> {
    let mut current = value;

    for part in path.split('.') {
        // Handle array access syntax [index]
        if part.contains('[') && part.ends_with(']') {
            let bracket_start = part.find('[').ok_or_else(|| {
                JsonExtractError::InvalidArrayIndex(format!("Invalid array syntax: {}", part))
            })?;
            let array_name = &part[..bracket_start];
            let index_str = &part[bracket_start + 1..part.len() - 1];
            let index = index_str.parse::<usize>().map_err(|_| {
                JsonExtractError::InvalidArrayIndex(format!("Invalid array index: {}", index_str))
            })?;

            // Get array from current value
            current = current
                .get(array_name)
                .ok_or_else(|| JsonExtractError::FieldNotFound(array_name.to_string()))?;
            let array = current
                .as_array()
                .ok_or_else(|| JsonExtractError::NotAnArray(array_name.to_string()))?;

            // Check index bounds
            if index >= array.len() {
                return Err(JsonExtractError::ArrayIndexOutOfBounds {
                    path: array_name.to_string(),
                    index,
                    length: array.len(),
                });
            }

            current = &array[index];
        } else {
            // Regular field access
            current = current
                .get(part)
                .ok_or_else(|| JsonExtractError::FieldNotFound(part.to_string()))?;
        }
    }

    Ok(current)
}

/// Format JSON value for output based on specified format
pub fn format_output(value: &JsonValue, output_format: Option<&str>) -> Result<String> {
    match output_format {
        Some("json-pretty") => Ok(serde_json::to_string_pretty(value)?),
        Some("raw") | None => Ok(value.to_string()),
        _ => Ok(serde_json::to_string(value)?),
    }
}

/// Strip surrounding quotes from a string if present
pub fn strip_quotes_internal(s: &str) -> String {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// Public API for stripping quotes
pub fn strip_quotes(s: &str) -> String {
    strip_quotes_internal(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_nested_value() {
        let json_value = json!({
            "package": {
                "name": "test",
                "dependencies": {
                    "serde": "1.0"
                }
            },
            "array": [1, 2, 3]
        });

        assert_eq!(
            get_nested_value(&json_value, "package.name").unwrap(),
            &json!("test")
        );

        assert_eq!(
            get_nested_value(&json_value, "package.dependencies.serde").unwrap(),
            &json!("1.0")
        );

        assert_eq!(
            get_nested_value(&json_value, "array[1]").unwrap(),
            &json!(2)
        );
    }

    #[test]
    fn test_strip_quotes_internal() {
        assert_eq!(strip_quotes_internal("\"hello\""), "hello");
        assert_eq!(strip_quotes_internal("'world'"), "world");
        assert_eq!(strip_quotes_internal("no_quotes"), "no_quotes");
    }
}
