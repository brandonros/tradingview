use miniserde::json::{Array, Number, Object, Value};
use simple_error::{box_err, SimpleResult};

pub fn value_to_string(input: &Value) -> SimpleResult<String> {
    match input {
        Value::String(value) => Ok(value.clone()),
        _ => Err(box_err!("parsing failed"))
    }
}

pub fn value_to_array(input: &Value) -> SimpleResult<Array> {
    match input {
        Value::Array(value) => Ok(value.clone()),
        _ => Err(box_err!("parsing failed"))
    }
}

pub fn value_to_object(input: &Value) -> SimpleResult<Object> {
    match input {
        Value::Object(value) => Ok(value.clone()),
        _ => Err(box_err!("parsing failed"))
    }
}

pub fn value_to_number(input: &Value) -> SimpleResult<Number> {
    match input {
        Value::Number(value) => Ok(value.clone()),
        _ => Err(box_err!("parsing failed"))
    }
}

pub fn value_to_f64_cast(input: &Value) -> SimpleResult<f64> {
    match input {
        Value::Number(value) => {
            match value {
                Number::U64(u64_value) => Ok(u64_value.clone() as f64),
                Number::I64(i64_value) => Ok(i64_value.clone() as f64),
                Number::F64(f64_value) => Ok(f64_value.clone())
            }
        },
        _ => Err(box_err!("parsing failed"))
    }
}

pub fn value_to_u64_cast(input: &Value) -> SimpleResult<u64> {
    match input {
        Value::Number(value) => {
            match value {
                Number::U64(u64_value) => Ok(u64_value.clone()),
                Number::I64(i64_value) => Ok(i64_value.clone() as u64),
                Number::F64(f64_value) => Ok(f64_value.clone() as u64)
            }
        },
        _ => Err(box_err!("parsing failed"))
    }
}

pub fn value_to_f64(input: &Value) -> SimpleResult<f64> {
    match input {
        Value::Number(value) => {
            match value {
                Number::F64(value) => Ok(value.clone()),
                _ => Err(box_err!("parsing failed"))
            }
        },
        _ => Err(box_err!("parsing failed"))
    }
}

pub fn value_to_u64(input: &Value) -> SimpleResult<u64> {
    match input {
        Value::Number(value) => {
            match value {
                Number::U64(value) => Ok(value.clone()),
                _ => Err(box_err!("parsing failed"))
            }
        },
        _ => Err(box_err!("parsing failed"))
    }
}

pub fn value_to_bool(input: &Value) -> SimpleResult<bool> {
    match input {
        Value::Bool(value) => Ok(value.clone()),
        _ => Err(box_err!("parsing failed"))
    }
}

pub fn is_null(input: &Object, key: &str) -> SimpleResult<bool> {
    let value = input.get(key).ok_or(box_err!("failed to get key"))?;
    match value {
        Value::Null => Ok(true),
        _ => Ok(false),
    }
}
