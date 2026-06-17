use crate::domain::contracts::JsonSchemaOutput;
use crate::domain::errors::SieveError;
use serde_json::Value;

pub(crate) fn validate_against_schema<T: JsonSchemaOutput>(
    value: &Value,
) -> Result<(), SieveError> {
    let schema = T::output_schema();
    validate_schema(schema.as_value(), value, "$")
}

/// Walks a JSON Schema (schemars 1.x renders schemas as plain JSON values) and
/// validates `value` against it. Mirrors the surface the typed schemars 0.8
/// walker covered: `type`, `required`, `properties`, and array `items`.
fn validate_schema(schema: &Value, value: &Value, path: &str) -> Result<(), SieveError> {
    match schema {
        // A boolean schema accepts (`true`) or rejects (`false`) everything.
        Value::Bool(true) => Ok(()),
        Value::Bool(false) => Err(violation(path, "value rejected by schema")),
        Value::Object(object) => {
            if let Some(instance_type) = object.get("type") {
                validate_instance_type(instance_type, value, path)?;
            }
            if object.contains_key("required") || object.contains_key("properties") {
                let Some(map) = value.as_object() else {
                    return Err(violation(path, "expected object"));
                };
                if let Some(Value::Array(required)) = object.get("required") {
                    for key in required.iter().filter_map(Value::as_str) {
                        if !map.contains_key(key) {
                            return Err(violation(
                                &format!("{path}.{key}"),
                                "missing required property",
                            ));
                        }
                    }
                }
                if let Some(Value::Object(properties)) = object.get("properties") {
                    for (key, nested_schema) in properties {
                        if let Some(nested_value) = map.get(key) {
                            validate_schema(nested_schema, nested_value, &format!("{path}.{key}"))?;
                        }
                    }
                }
            }
            if let Some(items) = object.get("items") {
                let Some(elements) = value.as_array() else {
                    return Err(violation(path, "expected array"));
                };
                for (index, item) in elements.iter().enumerate() {
                    validate_array_item(items, item, &format!("{path}[{index}]"))?;
                }
            }
            Ok(())
        }
        // Any other JSON node is not a recognizable schema; nothing to enforce.
        _ => Ok(()),
    }
}

/// `type` may be a single string or an array of strings (a union); the value
/// must match at least one.
fn validate_instance_type(
    instance_type: &Value,
    value: &Value,
    path: &str,
) -> Result<(), SieveError> {
    let matches = match instance_type {
        Value::String(kind) => matches_instance(value, kind),
        Value::Array(kinds) => kinds
            .iter()
            .filter_map(Value::as_str)
            .any(|kind| matches_instance(value, kind)),
        _ => true,
    };
    if matches {
        Ok(())
    } else {
        Err(violation(path, "value does not match expected type"))
    }
}

fn matches_instance(value: &Value, instance_type: &str) -> bool {
    match instance_type {
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

/// `items` may be a single schema or an array of schemas (tuple validation);
/// match the typed walker by validating each element against the first.
fn validate_array_item(items: &Value, value: &Value, path: &str) -> Result<(), SieveError> {
    match items {
        Value::Array(list) => match list.first() {
            Some(first) => validate_schema(first, value, path),
            None => Ok(()),
        },
        other => validate_schema(other, value, path),
    }
}

fn violation(path: &str, message: &str) -> SieveError {
    SieveError::SchemaViolation {
        path: path.to_owned(),
        message: message.to_owned(),
    }
}
