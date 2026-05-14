package orchestration

import (
	"testing"
	"time"
)

func TestModels(t *testing.T) {
	now := time.Now()

	tenant := Tenant{
		ID:         "t1",
		OwnerEmail: "test@example.com",
		Tier:       "pro",
		CreatedAt:  now,
	}

	if tenant.ID != "t1" || tenant.OwnerEmail != "test@example.com" {
		t.Errorf("Tenant struct mismatch")
	}

	business := Business{
		ID:        "b1",
		TenantID:  "t1",
		Name:      "Test Biz",
		Type:      "Retail",
		CreatedAt: now,
		UpdatedAt: now,
	}

	if business.Name != "Test Biz" || business.Type != "Retail" {
		t.Errorf("Business struct mismatch")
	}

	memory := AgentMemory{
		ID:         "m1",
		TenantID:   "t1",
		BusinessID: "b1",
		Department: "Sales",
		Embeddings: []float32{0.1, 0.2, 0.3},
		CreatedAt:  now,
	}

	if memory.Department != "Sales" || len(memory.Embeddings) != 3 {
		t.Errorf("AgentMemory struct mismatch")
	}
}
