// Only the focused regression harness includes this adapter.
// It calls the maintained catalog conversion and transactional save routines.
impl OnboardingAgent {
    async fn create_product(
        &self,
        org_id: &str,
        product: &IntakeProduct,
        price_type: &str,
        business_type: &str,
        deposit_percentage: Option<i32>,
        lead_time_days: Option<i32>,
    ) -> Result<(), String> {
        let product = Self::catalog_product(
            product,
            price_type,
            business_type,
            deposit_percentage,
            lead_time_days,
            Default::default(),
        )
        .map_err(|e| e.to_string())?;
        let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
        crate::common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;
        crate::preparation::save_catalog(&mut tx, org_id, &[product])
            .await
            .map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
