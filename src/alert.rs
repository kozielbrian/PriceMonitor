use serde::Serialize;

/// A struct representing the JSON output

#[derive(Serialize)]
#[allow(dead_code)]
pub enum LogLevels {
    Info,
    Error,
    Debug,
}

#[derive(Serialize)]
pub struct AlertData {
    pub last_price: String,
    pub average: String,
    pub change: String,
    pub sdev: String,
}

#[derive(Serialize)]
pub struct Alert {
    pub timestamp: String,
    pub level: LogLevels,
    pub trading_pair: String,
    pub deviation: bool,
    pub data: AlertData,
}
