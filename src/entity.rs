use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    status: usize,
    message: String,
    data: Option<T>,
}

impl ApiResponse<()> {
    pub fn new(status: usize, message: String) -> Self {
        ApiResponse {
            status,
            message,
            data: None,
        }
    }
}

impl<T: Serialize> ApiResponse<T> {
    pub fn data(status: usize, message: String, data: T) -> Self {
        ApiResponse {
            status: status,
            message: message,
            data: Some(data),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(with = "jwt")]
    pub exp: DateTime<Utc>,
}

mod jwt {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(data: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = data.timestamp();
        s.serialize_i64(timestamp)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single() // If there are multiple or no valid DateTimes from timestamp, return None
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}
