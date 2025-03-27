use axum::extract::Path;

use crate::model::user::UserClaim;

pub async fn delete(user_claim: UserClaim, Path(user_id): Path<String>) {}
