package billing

import (
	"context"
	"testing"
)

func TestProcessUsage(t *testing.T) {
	svc := NewBillingService()
	err := svc.ProcessUsage(context.Background(), 1000, 500)
	if err != nil {
		t.Errorf("ProcessUsage failed: %v", err)
	}
}
