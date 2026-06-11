package saga

import (
    "context"
    "testing"
)

func TestExampleMultiStepSaga_Success(t *testing.T) {
    db := &mockDB{}
    coordinator := NewCoordinator(db)
    coordinator.SyncExecution = true

    saga := NewExampleMultiStepSaga()
    coordinator.Register(saga)

    _, err := coordinator.Start(context.Background(), saga.Name, map[string]interface{}{})
    if err != nil {
        t.Fatalf("Failed to start saga: %v", err)
    }
}

func TestExampleMultiStepSaga_FailureAndCompensation(t *testing.T) {
    db := &mockDB{}
    coordinator := NewCoordinator(db)
    coordinator.SyncExecution = true

    saga := NewExampleMultiStepSaga()
    coordinator.Register(saga)

    _, err := coordinator.Start(context.Background(), saga.Name, map[string]interface{}{
        "fail_inventory": true,
    })
    if err != nil {
        t.Fatalf("Failed to start saga: %v", err)
    }
}
