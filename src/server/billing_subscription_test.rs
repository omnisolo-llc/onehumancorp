use crate::billing_subscription::{SubscriptionEngine};

#[tokio::test]
async fn test_subscription_lifecycle() {
    let engine = SubscriptionEngine::new();
    let tenant_id = "tenant_1";
    let customer_id = "cust_1";

    let plan = engine.create_plan(tenant_id, "Guitar Lessons", 10000).await;
    assert_eq!(plan.name, "Guitar Lessons");
    assert_eq!(plan.amount_cents, 10000);

    let sub = engine.subscribe_customer(tenant_id, customer_id, &plan.id).await;
    assert_eq!(sub.status, "active");

    engine.handle_payment_failed(tenant_id, &sub.id).await;
    let subs = engine.subscriptions.lock().await;
    let updated_sub = subs.get(&sub.id).unwrap();
    assert_eq!(updated_sub.status, "past_due");
    drop(subs);

    let processed_actions = engine.process_dunning().await;
    assert_eq!(processed_actions.len(), 1);
    assert_eq!(processed_actions[0].subscription_id, sub.id);
    assert_eq!(processed_actions[0].status, "email_sent");
}
