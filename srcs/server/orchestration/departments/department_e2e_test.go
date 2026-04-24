package departments

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// A simple test for Department framework functionality.
func TestDepartmentFramework(t *testing.T) {
	pool := db.NewTestProvider(t)

	database := &db.DB{Provider: pool}
	if err := database.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	ctx := context.Background()

	// Implement simple mockup of an AI agent department
	manager := NewManager()

	mockDept := &mockDepartment{
		handledEvents: make([]string, 0),
		draftsEmitted: make([]string, 0),
	}

	manager.RegisterDepartment("Operations", mockDept)

	// Simulate event reception
	dept, err := manager.GetDepartment("Operations")
	if err != nil {
		t.Fatalf("failed to get department: %v", err)
	}

	payload := map[string]string{"order_id": "1234"}
	payloadBytes, _ := json.Marshal(payload)

	err = dept.HandleEvent(ctx, "tenant-123", "order.created", payloadBytes)
	if err != nil {
		t.Fatalf("HandleEvent failed: %v", err)
	}

	if len(mockDept.handledEvents) != 1 {
		t.Fatalf("expected 1 handled event, got %d", len(mockDept.handledEvents))
	}

	// Draft action
	err = dept.EmitDraftAction(ctx, "tenant-123", "confirm_order", payloadBytes)
	if err != nil {
		t.Fatalf("EmitDraftAction failed: %v", err)
	}

	if len(mockDept.draftsEmitted) != 1 {
		t.Fatalf("expected 1 draft action, got %d", len(mockDept.draftsEmitted))
	}
}

type mockDepartment struct {
	handledEvents []string
	draftsEmitted []string
}

func (m *mockDepartment) HandleEvent(ctx context.Context, tenantID, eventType string, payload []byte) error {
	m.handledEvents = append(m.handledEvents, eventType)
	return nil
}

func (m *mockDepartment) RetrieveMemoryContext(ctx context.Context, tenantID, query string, limit int) ([]string, error) {
	return []string{"mocked context"}, nil
}

func (m *mockDepartment) EmitDraftAction(ctx context.Context, tenantID, actionType string, details []byte) error {
	m.draftsEmitted = append(m.draftsEmitted, actionType)
	return nil
}
