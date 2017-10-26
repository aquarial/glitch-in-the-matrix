use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Receipt {
    pub ts: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Receipts {
    #[serde(rename="m.read")]
    pub read: HashMap<String,Receipt>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
/// Information about whether people are online or not.
pub enum Presence {
    Online,
    Offline,
    Unavailable
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomTag {
    // can be a number or a string
    pub order: Option<::serde_json::Value>,
}

