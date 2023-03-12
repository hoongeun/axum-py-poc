use serde::Serialize;
use serde_json::Json as JsonValue;

#[derive(Serialize)]
pub struct Prop {
    pub value: JsonValue,
}
