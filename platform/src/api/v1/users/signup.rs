#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;

use serde::{Deserialize, Serialize};

use crate::{api::common::TypeField, models::user::User, utils::hash::get_hash_string};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignupUserRequest {
    pub auth_key: String,
    pub email: String,
    pub password: String,
}

#[server(endpoint = "/v1/users/signup", input = Json, output = Json)]
pub async fn signup_user(req: SignupUserRequest) -> Result<(), ServerFnError> {
    use crate::api::aws::ses::AuthKeyModel;
    use easy_dynamodb::error::DynamoException;

    let log = crate::utils::logger::new_api("POST", &format!("/v1/users/signup"));
    let cli = crate::utils::db::get(&log);

    let auth_key_model: Result<(Option<Vec<AuthKeyModel>>, Option<String>), DynamoException> = cli
        .find(
            "auth-key-index",
            None,
            Some(1),
            vec![("auth_key", req.auth_key)],
        )
        .await;

    match auth_key_model {
        Ok(v) => {
            let auth_model_vec = &v.0.unwrap();

            if auth_model_vec.len() == 0 {
                return Err(ServerFnError::ServerError(format!(
                    "Auth key is not exists"
                )));
            }

            let auth_model = &auth_model_vec[0];

            let id = auth_model.id.clone();

            match cli
                .update::<TypeField>(id.as_str(), vec![("is_used", TypeField::N(2))])
                .await
            {
                Ok(()) => {
                    tracing::debug!("auth key update success");
                }
                Err(e) => {
                    return Err(ServerFnError::ServerError(format!("update failed {e}")));
                }
            }
        }
        Err(_e) => {
            return Err(ServerFnError::ServerError(format!(
                "Auth key is not exists"
            )));
        }
    }

    let pw_hash = get_hash_string(req.password.as_bytes());
    let user = User {
        id: req.email.clone(),
        email: req.email.clone(),
        hashed_password: pw_hash,
    };

    match cli.get::<User>(req.email.as_str()).await {
        Ok(v) => match v {
            Some(_) => {
                return Err(ServerFnError::ServerError(format!("already exists user")));
            }
            None => match cli.create(user).await {
                Ok(()) => {
                    tracing::debug!("create user success",);
                    return Ok(());
                }
                Err(e) => {
                    return Err(ServerFnError::ServerError(format!("find user failed {e}")));
                }
            },
        },
        Err(_e) => match cli.create(user).await {
            Ok(()) => {
                tracing::debug!("create user success",);
                Ok(())
            }
            Err(e) => {
                return Err(ServerFnError::ServerError(format!(
                    "create user failed {e}"
                )));
            }
        },
    }
}
