package departments

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/orchestration"
)

// MockHub implements HubPublisher for testing
type MockHub struct {
	handlers []func(orchestration.Message)
}

func (m *MockHub) Publish(msg orchestration.Message) error {
	for _, h := range m.handlers {
		h(msg)
	}
	return nil
}

func (m *MockHub) RegisterHandler(h func(orchestration.Message)) {
	m.handlers = append(m.handlers, h)
}

// MockMemoryLayer simulates the pgvector shared memory for testing.
type MockMemoryLayer struct {
	mu     sync.Mutex
	events []orchestration.Message
}

func (m *MockMemoryLayer) SaveEvent(ctx context.Context, event orchestration.Message) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.events = append(m.events, event)
	return nil
}

func (m *MockMemoryLayer) Retrieve(ctx context.Context, query string) (*MemoryContext, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	return &MemoryContext{
		RelevantEvents: m.events, // Return all in mock
		Context:        "Simulated context for query: " + query,
	}, nil
}

// MockActionReviewCenter simulates the UI Action Review Center for testing.
type MockActionReviewCenter struct {
	mu      sync.Mutex
	actions []DraftAction
}

func (arc *MockActionReviewCenter) ReceiveDraft(ctx context.Context, action DraftAction) error {
	arc.mu.Lock()
	defer arc.mu.Unlock()
	arc.actions = append(arc.actions, action)
	return nil
}

func (arc *MockActionReviewCenter) GetDrafts() []DraftAction {
	arc.mu.Lock()
	defer arc.mu.Unlock()
	return arc.actions
}

func TestAgentDepartmentIntegration(t *testing.T) {
	ctx := context.Background()

	// Initialize shared components
	memoryLayer := &MockMemoryLayer{}
	reviewCenter := &MockActionReviewCenter{}
	hub := &MockHub{}

	// Initialize Departments
	opsDept := &OperationsDepartment{
		BaseDepartment: NewBaseDepartment("ops-1", DepartmentOperations, memoryLayer, reviewCenter, "agent-manager"),
	}
	csDept := &CustomerSuccessDepartment{
		BaseDepartment: NewBaseDepartment("cs-1", DepartmentSuccess, memoryLayer, reviewCenter, "agent-ambassador"),
	}

	opsDept.Start(ctx, hub)
	csDept.Start(ctx, hub)

	// Simulate event routing
	hub.RegisterHandler(func(msg orchestration.Message) {
		// Route based on type
		if msg.Type == "order.created" {
			opsDept.HandleEvent(ctx, msg)
		} else if msg.Type == "order.processed" {
			csDept.HandleEvent(ctx, msg)
		}
	})

	// Trigger: E2E Flow starts
	// Simulate "order.created" event from UI
	orderEvent := orchestration.Message{
		ID:         "order-123",
		Type:       "order.created",
		Content:    "New custom cake order",
		OccurredAt: time.Now().UTC(),
	}

	hub.Publish(orderEvent)

	// Verify
	drafts := reviewCenter.GetDrafts()
	if len(drafts) != 1 {
		t.Fatalf("Expected 1 draft action, got %d", len(drafts))
	}

	draft := drafts[0]
	if draft.DepartmentType != DepartmentSuccess {
		t.Errorf("Expected draft from %s, got %s", DepartmentSuccess, draft.DepartmentType)
	}
	if draft.ActionType != "send_confirmation_message" {
		t.Errorf("Expected action 'send_confirmation_message', got %s", draft.ActionType)
	}
	if draft.Payload["message"] == "" {
		t.Errorf("Expected non-empty draft message")
	}

	// Verify memory integration
	if len(memoryLayer.events) != 1 {
		t.Errorf("Expected 1 event in memory, got %d", len(memoryLayer.events))
	}
}
