use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::Reducible;

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct UserInfo {
    pub auth_code: Option<String>,
    pub name: String,
    pub email: String,
}

impl Reducible for UserInfo {
    type Action = UserInfo;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}
