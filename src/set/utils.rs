//! Utility functions for set operations

use crate::error::JsonExtractError;
use serde_json::Value as JsonValue;

/// Split field path into segments (handles array syntax like "arr\[0\]")
pub fn split_field_path(field_path: &str) -> Result<Vec<String>, JsonExtractError> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_array = false;

    for c in field_path.chars() {
        match c {
            '.' if !in_array => {
                if current.is_empty() {
                    return Err(JsonExtractError::InvalidFieldPath(
                        "Empty path segment".to_string(),
                    ));
                }
                parts.push(current);
                current = String::new();
            }
            '[' => {
                in_array = true;
                current.push(c);
            }
            ']' => {
                in_array = false;
                current.push(c);
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    } else {
        return Err(JsonExtractError::InvalidFieldPath(
            "Field path cannot end with a dot".to_string(),
        ));
    }

    Ok(parts)
}

/// Parse value with optional type hint
pub fn parse_value_with_type(
    value: &str,
    value_type: Option<&str>,
) -> Result<JsonValue, JsonExtractError> {
    match value_type {
        Some("string") => Ok(JsonValue::String(value.to_string())),
        Some("integer") => value.parse::<i64>().map(JsonValue::from).map_err(|_| {
            JsonExtractError::InvalidValueType(format!("{} is not a valid integer", value))
        }),
        Some("float") => value.parse::<f64>().map(JsonValue::from).map_err(|_| {
            JsonExtractError::InvalidValueType(format!("{} is not a valid float", value))
        }),
        Some("boolean") => match value.to_lowercase().as_str() {
            "true" => Ok(JsonValue::Bool(true)),
            "false" => Ok(JsonValue::Bool(false)),
            _ => Err(JsonExtractError::InvalidValueType(format!(
                "{} is not a valid boolean",
                value
            ))),
        },
        Some("null") => Ok(JsonValue::Null),
        _ => {
            // Try to parse as JSON first
            if let Ok(json_value) = serde_json::from_str(value) {
                Ok(json_value)
            } else {
                // Auto-detect type
                if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
                    Ok(JsonValue::Bool(value.eq_ignore_ascii_case("true")))
                } else if value.eq_ignore_ascii_case("null") {
                    Ok(JsonValue::Null)
                } else if let Ok(num) = value.parse::<i64>() {
                    Ok(JsonValue::Number(num.into()))
                } else if let Ok(num) = value.parse::<f64>() {
                    // Check if it's actually an integer
                    if num.fract() == 0.0 && num.abs() < 2.0f64.powi(53) {
                        Ok(JsonValue::Number((num as i64).into()))
                    } else {
                        Ok(JsonValue::Number(
                            serde_json::Number::from_f64(num).unwrap(),
                        ))
                    }
                } else {
                    Ok(JsonValue::String(value.to_string()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_field_path() {
        assert_eq!(
            split_field_path("package.name").unwrap(),
            vec!["package".to_string(), "name".to_string()]
        );

        assert_eq!(
            split_field_path("dependencies.serde.version").unwrap(),
            vec![
                "dependencies".to_string(),
                "serde".to_string(),
                "version".to_string()
            ]
        );
    }

    #[test]
    fn test_parse_value_with_type() {
        assert!(matches!(
            parse_value_with_type("42", Some("integer")).unwrap(),
            JsonValue::Number(n) if n.as_i64() == Some(42)
        ));
        assert!(matches!(
            parse_value_with_type("true", Some("boolean")).unwrap(),
            JsonValue::Bool(true)
        ));
        assert!(
            matches!(parse_value_with_type("text", Some("string")).unwrap(), JsonValue::String(s) if s == "text")
        );
    }
}
