#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ConsumeQuery {
    pub channel: String,
    pub amount: Option<usize>,
}

