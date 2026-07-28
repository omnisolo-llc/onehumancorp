use super::order_interceptor::{InterceptOrderRequest, InterceptedOrder, InterceptedOrderItem};

#[test]
fn test_intercepted_order_structs() {
    let req = InterceptOrderRequest {
        raw_input: "test".to_string(),
    };
    assert_eq!(req.raw_input, "test");

    let order_item = InterceptedOrderItem {
        item: "tacos".to_string(),
        quantity: 3,
    };
    assert_eq!(order_item.item, "tacos");
    assert_eq!(order_item.quantity, 3);

    let order = InterceptedOrder {
        intent: "Order".to_string(),
        items: vec![order_item],
        language: "es".to_string(),
        notes: Some("no onions".to_string()),
        translated_notes: Some("sin cebolla".to_string()),
    };

    assert_eq!(order.intent, "Order");
    assert_eq!(order.language, "es");
    assert_eq!(order.items.len(), 1);
    assert_eq!(order.items[0].item, "tacos");
    assert_eq!(order.items[0].quantity, 3);
}
