package billing

import (
	"testing"
)

func TestBillingService_BillCustomer(t *testing.T) {
	svc := NewBillingService()

	cost, err := svc.BillCustomer("customer-123", "claude-3-5-sonnet-20240620", 1000000, 1000000, 0)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if cost != 18.00 {
		t.Errorf("expected 18.00, got %v", cost)
	}

	_, err = svc.BillCustomer("", "gpt-4o", 100, 100, 0)
	if err == nil {
		t.Errorf("expected error for empty customer id")
	}
}
