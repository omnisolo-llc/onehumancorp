use crate::acon::AconContextManager;

#[test]
fn test_acon_context_manager() {
    let manager = AconContextManager::new();
    assert_eq!(manager.preserve_reasoning_traces, true);

    let result = manager.manage_context("test");
    assert_eq!(result, "Managed Context: test");
}
