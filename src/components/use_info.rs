use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct UserInfo {
    pub auth_code: String,
    pub name: String,
    pub email: String,
}
