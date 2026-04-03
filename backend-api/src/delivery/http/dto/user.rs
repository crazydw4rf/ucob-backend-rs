use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}
