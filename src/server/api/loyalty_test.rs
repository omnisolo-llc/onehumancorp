use super::*;
use std::sync::Arc;
use crate::db::DB;
use ::server_common::Claims;
use axum::{
    extract::{State, Path},
    Extension,
    Json,
};
use crate::api::loyalty::{CreateLoyaltyProgramRequest, EarnPointsRequest, create_program_handler, earn_points_handler};

#[tokio::test]
async fn test_create_program_unauthorized() {
    let pool = crate::db::get_pool();
    let db = Arc::new(DB::new(pool));

    let claims = Claims {
        sub: "user_1".to_string(),
        organization_id: None,
        exp: 10000000000,
        iat: 0,
        email: None,
    };

    let req = CreateLoyaltyProgramRequest {
        name: "Test Program".to_string(),
        program_type: "points".to_string(),
        config: "{}".to_string(),
    };

    let response = create_program_handler(
        State(db.clone()),
        Extension(claims),
        Json(req),
    ).await;

    let res = response.into_response();
    assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_earn_points_unauthorized() {
    let pool = crate::db::get_pool();
    let db = Arc::new(DB::new(pool));

    let claims = Claims {
        sub: "user_1".to_string(),
        organization_id: None,
        exp: 10000000000,
        iat: 0,
        email: None,
    };

    let req = EarnPointsRequest {
        account_id: "acct_1".to_string(),
        points: 10,
        punches: 0,
        reason: "Purchase".to_string(),
    };

    let response = earn_points_handler(
        State(db.clone()),
        Extension(claims),
        Json(req),
    ).await;

    let res = response.into_response();
    assert_eq!(res.status(), axum::http::StatusCode::UNAUTHORIZED);
}
