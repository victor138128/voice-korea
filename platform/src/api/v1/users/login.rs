#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};

use serde::{Deserialize, Serialize};

use crate::{models::user::User, utils::hash::get_hash_string};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}

#[server(endpoint = "/v1/users/login", input = Json, output = Json)]
pub async fn login_user(req: LoginUserRequest) -> Result<(), ServerFnError> {
    use easy_dynamodb::error::DynamoException;
    let log = crate::utils::logger::new_api("POST", &format!("/v1/users/signup"));
    let cli = crate::utils::db::get(&log);

    let pw_hash = get_hash_string(req.password.as_bytes());

    match cli.get::<User>(req.email.as_str()).await {
        Ok(v) => match v {
            Some(user) => {
                if user.email == req.email && user.hashed_password == pw_hash {
                    Ok(())
                } else {
                    return Err(ServerFnError::ServerError(format!("not matched")));
                }
            }
            None => {
                return Err(ServerFnError::ServerError(format!("not exists user")));
            }
        },
        Err(e) => {
            return Err(ServerFnError::ServerError(format!("login failed {:?}", e)));
        }
    }
}
