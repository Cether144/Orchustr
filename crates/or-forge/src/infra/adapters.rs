use crate::domain::errors::ForgeError;
use schemars::Schema;
use serde_json::Value;

/// Validates tool call arguments against the tool's input schema. Under
/// schemars 1.x a `Schema` is a thin wrapper over a JSON value, so we
/// introspect the JSON keywords directly, enforcing `type` and `required`
/// the same way the typed schemars 0.8 walker did.
pub(crate) fn validate_tool_args(schema: &Schema, value: &Value) -> Result<(), ForgeError> {
    // A boolean schema accepts (`true`) or rejects (`false`) everything.
    if let Some(accepts) = schema.as_bool() {
        return if accepts {
            Ok(())
        } else {
            Err(ForgeError::InvalidArguments(
                "schema rejected value".to_owned(),
            ))
        };
    }

    if let Some(instance_type) = schema.get("type") {
        validate_type(instance_type, value)?;
    }
    if let Some(Value::Array(required)) = schema.get("required") {
        let map = value.as_object().ok_or_else(|| {
            ForgeError::InvalidArguments("expected object arguments".to_owned())
        })?;
        for key in required.iter().filter_map(Value::as_str) {
            if !map.contains_key(key) {
                return Err(ForgeError::InvalidArguments(format!(
                    "missing required argument: {key}"
                )));
            }
        }
    }
    Ok(())
}

/// `type` may be a single string or an array of strings (a union); the value
/// must match at least one.
fn validate_type(types: &Value, value: &Value) -> Result<(), ForgeError> {
    let matches = match types {
        Value::String(kind) => instance_matches(kind, value),
        Value::Array(kinds) => kinds
            .iter()
            .filter_map(Value::as_str)
            .any(|kind| instance_matches(kind, value)),
        _ => true,
    };
    if matches {
        Ok(())
    } else {
        Err(ForgeError::InvalidArguments(
            "argument value does not match schema type".to_owned(),
        ))
    }
}

fn instance_matches(kind: &str, value: &Value) -> bool {
    match kind {
        "null" => value.is_null(),
        "boolean" => value.is_boolean(),
        "object" => value.is_object(),
        "array" => value.is_array(),
        "number" => value.is_number(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "string" => value.is_string(),
        _ => true,
    }
}
