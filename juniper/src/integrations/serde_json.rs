use crate::{InputValue,
    parser::{ParseError, ScalarToken, Token},
    ParseScalarResult
};
use serde_json::{Value, Map, Number};

#[crate::graphql_scalar(description = "json value")]
impl<S> GraphQLScalar for Value
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        convert_to_juniper_value(&self)
    }

    pub fn from_input_value(value: &juniper::InputValue) -> Option<Value> {
        value.as_string_value().and_then(|s| {
            serde_json::from_str::<serde_json::Value>(s).ok()
        })
    }

    fn from_str(value: juniper::ScalarToken) -> juniper::ParseScalarResult<S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

pub fn convert_to_juniper_value<S>(json: &serde_json::Value) -> juniper::Value<S>
where
    S: juniper::ScalarValue,
{
    match json {
        serde_json::Value::Null => juniper::Value::null(),
        serde_json::Value::Bool(b) => juniper::Value::scalar(*b),
        serde_json::Value::Number(n) => {
            if let Some(n) = n.as_u64() {
                juniper::Value::scalar(n as i32)
            } else if let Some(n) = n.as_i64() {
                juniper::Value::scalar(n as i32)
            } else if let Some(n) = n.as_f64() {
                juniper::Value::scalar(n)
            } else {
                unreachable!("serde_json::Number has only 3 number variants")
            }
        }
        serde_json::Value::String(s) => juniper::Value::scalar(s.clone()),
        serde_json::Value::Array(a) => {
            let arr = a
                .iter()
                .map(|v| convert_to_juniper_value(v))
                .collect::<Vec<_>>();
            juniper::Value::list(arr)
        }
        serde_json::Value::Object(o) => {
            let obj: juniper::Object<S> = o
                .iter()
                .map(|(k, v)| (k, convert_to_juniper_value(v)))
                .collect();
            juniper::Value::object(obj)
        }
    }
}
