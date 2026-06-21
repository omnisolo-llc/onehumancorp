#[cfg(test)]
mod tests {
    use super::service::LoyaltyServiceImpl;
    use loyalty_proto::ohc::loyalty::{
        loyalty_service_server::LoyaltyService,
        CreateLoyaltyProgramRequest, EarnPointsRequest, RedeemRewardRequest,
    };
    use tonic::Request;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_loyalty_program() {
        let pool = crate::db::get_pool();
        let service = LoyaltyServiceImpl::new(pool.clone());
        let tenant_id = Uuid::new_v4().to_string();

        let req = CreateLoyaltyProgramRequest {
            tenant_id: tenant_id.clone(),
            name: "Test Points Program".to_string(),
            program_type: "points".to_string(),
            config: "{}".to_string(),
        };

        let response = service.create_loyalty_program(Request::new(req)).await;
        assert!(response.is_ok());

        let res = response.unwrap().into_inner();
        let program = res.program.unwrap();
        assert_eq!(program.name, "Test Points Program");
        assert_eq!(program.program_type, "points");
        assert_eq!(program.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_earn_points_and_redeem_reward() {
        let pool = crate::db::get_pool();
        let service = LoyaltyServiceImpl::new(pool.clone());
        let tenant_id = Uuid::new_v4().to_string();
        let customer_id = Uuid::new_v4().to_string();
        let program_id = Uuid::new_v4().to_string();

        let req = EarnPointsRequest {
            tenant_id: tenant_id.clone(),
            customer_id: customer_id.clone(),
            program_id: program_id.clone(),
            points: 100,
            punches: 0,
            description: "Purchase points".to_string(),
        };

        let response = service.earn_points(Request::new(req)).await;
        assert!(response.is_ok());

        let res = response.unwrap().into_inner();
        let account = res.account.unwrap();
        assert_eq!(account.points_balance, 100);

        let tx = res.transaction.unwrap();
        assert_eq!(tx.points, 100);
        assert_eq!(tx.transaction_type, "earn");

        // Manually create a reward to test redemption
        let reward_id = Uuid::new_v4().to_string();
        let mut conn = pool.acquire().await.unwrap();
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *conn).await.unwrap();
        sqlx::query(
            "INSERT INTO rewards (id, tenant_id, program_id, name, points_cost, punches_cost, reward_type) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&reward_id)
        .bind(&tenant_id)
        .bind(&program_id)
        .bind("Free Coffee")
        .bind(50)
        .bind(0)
        .bind("free_item")
        .execute(&mut *conn)
        .await
        .unwrap();

        let redeem_req = RedeemRewardRequest {
            tenant_id: tenant_id.clone(),
            customer_id: customer_id.clone(),
            program_id: program_id.clone(),
            reward_id: reward_id.clone(),
        };

        let redeem_res = service.redeem_reward(Request::new(redeem_req)).await;
        assert!(redeem_res.is_ok());

        let redeem_res_inner = redeem_res.unwrap().into_inner();
        assert!(redeem_res_inner.success);
        assert_eq!(redeem_res_inner.account.unwrap().points_balance, 50);
        assert_eq!(redeem_res_inner.transaction.unwrap().points, -50);
    }
}
