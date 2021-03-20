use std::usize;

use actix_web::http::{header::Header, uri};
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

#[derive(Debug, Serialize)]
pub struct NovelMeateData {
    pub name: Option<String>,
    pub author: Option<String>,
    pub intro: Option<String>,
    pub lastchapter: Option<String>,
    pub cover: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChapterMetadata {
    pub name: Option<String>,
    pub serial: Option<usize>,
    pub url: Option<String>,
}

impl NovelMeateData {
    pub fn builder() -> Self {
        NovelMeateData {
            name: None,
            author: None,
            intro: None,
            lastchapter: None,
            cover: None,
            url: None,
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn author(&mut self, author: String) -> &mut Self {
        self.author = Some(author);
        self
    }

    pub fn intro(&mut self, intro: String) -> &mut Self {
        self.intro = Some(intro);
        self
    }

    pub fn lastchapter(&mut self, lastchapter: String) -> &mut Self {
        self.lastchapter = Some(lastchapter);
        self
    }

    pub fn cover(&mut self, cover: String) -> &mut Self {
        self.cover = Some(cover);
        self
    }

    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }
}

impl ChapterMetadata {
    pub fn builder() -> Self {
        ChapterMetadata {
            name: None,
            serial: None,
            url: None,
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn serial(&mut self, serial: usize) -> &mut Self {
        self.serial = Some(serial);
        self
    }

    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ReadQuery {
    pub url: String,
}
