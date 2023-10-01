#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Message {
    pub channel: String,
    pub headers: Option<serde_json::Value>,
    pub body: serde_json::Value,
}

