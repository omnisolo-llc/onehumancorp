#[cfg(test)]
mod tests {
    use ohc_rust_protos::loyalty::{CreateLoyaltyProgramRequest, EarnPointsRequest, CreateRewardRequest, RedeemRewardRequest};
    use crate::db::get_pool;
    use tonic::Request;
    use crate::services::loyalty::service::LoyaltyServiceImpl;
    use ohc_rust_protos::loyalty::loyalty_service_server::LoyaltyService;
    use crate::harness::TestHarness;

    #[tokio::test]
    async fn test_loyalty_service_crud_and_points() {
        let harness = TestHarness::new().await;
        let service = LoyaltyServiceImpl { pool: harness.pool.clone() };
        let tenant_id = "tenant_test_123".to_string();

        // 1. Create a Loyalty Program
        let create_req = CreateLoyaltyProgramRequest {
            tenant_id: tenant_id.clone(),
            name: "Test Points Program".to_string(),
            program_type: "points".to_string(),
            config_json: "{}".to_string(),
        };
        let create_res = service.create_loyalty_program(Request::new(create_req)).await.unwrap().into_inner();
        assert_eq!(create_res.name, "Test Points Program");
        let program_id = create_res.id;

        // 2. Earn points (this should auto-create the account)
        let earn_req = EarnPointsRequest {
            tenant_id: tenant_id.clone(),
            program_id: program_id.clone(),
            customer_id: "customer_123".to_string(),
            points: 100,
            punches: 0,
            reason: "Initial signup".to_string(),
            order_id: "".to_string(),
        };
        let earn_res = service.earn_points(Request::new(earn_req)).await.unwrap().into_inner();
        assert_eq!(earn_res.points_balance, 100);
        let account_id = earn_res.id;

        // 3. Create a Reward
        let reward_req = CreateRewardRequest {
            tenant_id: tenant_id.clone(),
            program_id: program_id.clone(),
            name: "Free Coffee".to_string(),
            description: "Get a free coffee".to_string(),
            points_cost: 50,
            reward_type: "free_item".to_string(),
            reward_value_json: "{}".to_string(),
        };
        let reward_res = service.create_reward(Request::new(reward_req)).await.unwrap().into_inner();
        let reward_id = reward_res.id;

        // 4. Redeem Reward
        let redeem_req = RedeemRewardRequest {
            tenant_id: tenant_id.clone(),
            account_id: account_id.clone(),
            reward_id: reward_id.clone(),
        };
        let redeem_res = service.redeem_reward(Request::new(redeem_req)).await.unwrap().into_inner();
        assert!(redeem_res.success);

        // 5. Verify remaining points
        let get_req = ohc_rust_protos::loyalty::GetCustomerAccountRequest {
            tenant_id: tenant_id.clone(),
            program_id: program_id.clone(),
            customer_id: "customer_123".to_string(),
        };
        let final_account = service.get_customer_account(Request::new(get_req)).await.unwrap().into_inner();
        assert_eq!(final_account.points_balance, 50);

        // 6. Transaction immutability check
        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM loyalty_transactions WHERE account_id = $1")
            .bind(&account_id)
            .fetch_one(&harness.pool)
            .await.unwrap();
        // Should be 2 transactions: 1 earn, 1 redeem
        assert_eq!(count.0, 2);
    }
}
