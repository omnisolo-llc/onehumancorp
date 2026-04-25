package autodream

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/src/server/db"
)

func TestKairosAutoDreamWorker_Postgres(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	// Just verifying that it runs on a regular DB
	worker := NewKairosAutoDreamWorker(provider, &MockWorkerLLMClient{})
	err := worker.RunConsolidation(ctx)
	if err != nil {
		t.Fatalf("RunConsolidation failed: %v", err)
	}
}

func TestAutoDreamWorker_Postgres(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	worker := NewAutoDreamWorker(provider, &MockWorkerLLMClient{})
	err := worker.RunConsolidation(ctx)
	if err != nil {
		t.Fatalf("RunConsolidation failed: %v", err)
	}
}
