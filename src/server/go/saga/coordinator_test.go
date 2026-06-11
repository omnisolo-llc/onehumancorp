package saga

import (
    "context"
    "database/sql"
    "errors"
    "testing"
)

// A simple mock DB implementation of db methods we need to test Coordinator logic
type mockDB struct {
    execs []string
}

func (m *mockDB) ExecContext(ctx context.Context, query string, args ...interface{}) (sql.Result, error) {
    m.execs = append(m.execs, query)
    return nil, nil
}

func (m *mockDB) QueryRowContext(ctx context.Context, query string, args ...interface{}) *sql.Row {
    return nil
}

func TestSagaCoordinator_Success(t *testing.T) {
    db := &mockDB{}
    coordinator := NewCoordinator(db)
    coordinator.SyncExecution = true // Run synchronously for tests

    step1Executed := false
    step2Executed := false

    saga := &Saga{
        Name: "TestSaga",
        Steps: []Step{
            {
                Name: "Step1",
                Action: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    step1Executed = true
                    return nil
                },
                Compensate: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    return nil
                },
            },
            {
                Name: "Step2",
                Action: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    step2Executed = true
                    return nil
                },
                Compensate: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    return nil
                },
            },
        },
    }

    coordinator.Register(saga)

    _, err := coordinator.Start(context.Background(), "TestSaga", map[string]interface{}{"key": "value"})
    if err != nil {
        t.Fatalf("Failed to start saga: %v", err)
    }

    if !step1Executed {
        t.Error("Step1 was not executed")
    }
    if !step2Executed {
        t.Error("Step2 was not executed")
    }
}

func TestSagaCoordinator_FailureAndCompensation(t *testing.T) {
    db := &mockDB{}
    coordinator := NewCoordinator(db)
    coordinator.SyncExecution = true

    step1Executed := false
    step1Compensated := false
    step2Executed := false

    saga := &Saga{
        Name: "TestSagaFail",
        Steps: []Step{
            {
                Name: "Step1",
                Action: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    step1Executed = true
                    return nil
                },
                Compensate: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    step1Compensated = true
                    return nil
                },
            },
            {
                Name: "Step2",
                Action: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    step2Executed = true
                    return errors.New("simulated failure")
                },
                Compensate: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    t.Error("Step2 should not be compensated because it failed")
                    return nil
                },
            },
        },
    }

    coordinator.Register(saga)

    _, err := coordinator.Start(context.Background(), "TestSagaFail", map[string]interface{}{})
    if err != nil {
        t.Fatalf("Failed to start saga: %v", err)
    }

    if !step1Executed {
        t.Error("Step1 was not executed")
    }
    if !step2Executed {
        t.Error("Step2 was not executed")
    }
    if !step1Compensated {
        t.Error("Step1 was not compensated")
    }
}
