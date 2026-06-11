package saga

import (
    "context"
    "fmt"
    "log"
)

// NewExampleMultiStepSaga creates a sample saga demonstrating forward execution and compensation.
func NewExampleMultiStepSaga() *Saga {
    return &Saga{
        Name: "ExampleMultiStepSaga",
        Steps: []Step{
            {
                Name: "BookCalendar",
                Action: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    log.Printf("Executing BookCalendar for Saga %d", sagaID)
                    return nil
                },
                Compensate: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    log.Printf("Compensating BookCalendar for Saga %d", sagaID)
                    return nil
                },
            },
            {
                Name: "ProcessPayment",
                Action: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    log.Printf("Executing ProcessPayment for Saga %d", sagaID)
                    // Simulate a failure if requested in data
                    if fail, ok := data["fail_payment"].(bool); ok && fail {
                        return fmt.Errorf("simulated payment failure")
                    }
                    return nil
                },
                Compensate: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    log.Printf("Compensating ProcessPayment (Refund) for Saga %d", sagaID)
                    return nil
                },
            },
            {
                Name: "UpdateInventory",
                Action: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    log.Printf("Executing UpdateInventory for Saga %d", sagaID)
                    // Simulate a failure if requested in data
                    if fail, ok := data["fail_inventory"].(bool); ok && fail {
                        return fmt.Errorf("simulated inventory update failure")
                    }
                    return nil
                },
                Compensate: func(ctx context.Context, sagaID int64, data map[string]interface{}) error {
                    log.Printf("Compensating UpdateInventory for Saga %d", sagaID)
                    return nil
                },
            },
        },
    }
}
