#[cfg(test)]
mod tests {
    use crate::p2p_sync::{P2PMeshNode, P2PMessage, CRDTState};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_crdt_convergence() {
        let node_a = P2PMeshNode::new();
        let node_b = P2PMeshNode::new();

        // Node A has inventory
        {
            let mut state = node_a.state.write().await;
            state.inventory.insert("item1".to_string(), 10);
            state.logical_clock = 1;
            state.timestamp = 100;
        }

        // Node B is empty

        // Node A decrements
        node_a.process_inventory_decrement("item1", 2).await;

        // Node A sends state to Node B
        let state_a = node_a.state.read().await.clone();
        let msg = P2PMessage {
            device_id: node_a.device_id.clone(),
            state: state_a,
        };

        node_b.receive_sync(msg).await;

        let state_b = node_b.state.read().await;
        assert_eq!(state_b.inventory.get("item1"), Some(&8));
    }
}
