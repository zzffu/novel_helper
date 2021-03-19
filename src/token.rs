use crate::entity::Claims;
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};

const KEY: &str = "zzdafdfdsfds";

pub fn encodetoken() -> Result<String> {
    let utc = Utc::now() + chrono::Duration::days(1);

    let claims = Claims { exp: utc };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(KEY.as_ref()),
    )
}

pub fn decodetoken(token: &str) -> Result<TokenData<Claims>> {
    let valid = Validation::default();
    decode::<Claims>(token, &DecodingKey::from_secret(KEY.as_ref()), &valid)
}
