use crate::ryz::err::res;
use crate::ryz::res::Res;
use crate::ryz::time::{utc, Time};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use jwt::{SignWithKey, ToBase64};
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserTokenPayload {
    pub user_id: i32,
    /// When the token was created.
    ///
    /// We store this field instead of `exp` to allow receiver to freely
    /// interpret using known exp delta.
    pub created: Time,
}

impl Expire for UserTokenPayload {
    fn get_created(&self) -> Res<Time> {
        Ok(*&self.created)
    }
}

pub trait Expire {
    fn get_created(&self) -> Res<Time>;

    /// Checks if for the given delta the object is considered as expired.
    ///
    /// Returns exp time with relation to the given delta.
    fn check_exp(&self, delta: Time) -> Res<Time> {
        let created = *&self.get_created().unwrap();
        let exp = created + delta;
        if exp < utc() {
            return res("exp_err", "expired token");
        }
        Ok(exp)
    }
}

pub fn new_token(
    payload: &(impl ToBase64 + Expire + Serialize),
    secret: &[u8],
) -> Res<String> {
    let encoded_secret: Hmac<Sha256> =
        Hmac::new_from_slice(secret.into()).unwrap();
    Ok(payload.sign_with_key(&encoded_secret).unwrap())
}

pub fn verify_token<T>(token: &String, secret: &[u8]) -> Res<T>
where
    T: ToBase64 + Expire + for<'a> Deserialize<'a>,
{
    let encoded_secret: Hmac<Sha256> = Hmac::new_from_slice(secret).unwrap();
    let payload: T = token.verify_with_key(&encoded_secret).unwrap();
    payload.check_exp(Time::from(60 * 60 * 24 * 365))?;
    Ok(payload)
}

pub fn new_rt(user_id: i32) -> Res<String> {
    let payload = UserTokenPayload {
        user_id: user_id,
        created: utc(),
    };
    new_token(&payload, b"weloveauth")
}

pub fn new_at(user_id: i32) -> Res<String> {
    let payload = UserTokenPayload {
        user_id: user_id,
        created: utc(),
    };
    new_token(&payload, b"helloworld")
}

pub fn verify_rt(rt: &String) -> Res<UserTokenPayload> {
    verify_token(rt, b"weloveauth")
}
