package hybridcrdtmcp

import (
	"context"
	"encoding/json"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type MockCRDTProvider struct {
	pullData map[string]json.RawMessage
	pushErr  error
	pullErr  error
	mergeErr error
}

func (m *MockCRDTProvider) Pull(ctx context.Context, claims *auth.Claims, entityID string) (json.RawMessage, error) {
	if m.pullErr != nil {
		return nil, m.pullErr
	}
	if data, ok := m.pullData[entityID]; ok {
		return data, nil
	}
	return nil, errors.New("not found")
}

func (m *MockCRDTProvider) Push(ctx context.Context, claims *auth.Claims, entityID string, stateVector json.RawMessage) error {
	if m.pushErr != nil {
		return m.pushErr
	}
	if m.pullData == nil {
		m.pullData = make(map[string]json.RawMessage)
	}
	m.pullData[entityID] = stateVector
	return nil
}

func (m *MockCRDTProvider) Merge(ctx context.Context, localVector json.RawMessage, remoteVector json.RawMessage) (json.RawMessage, error) {
	if m.mergeErr != nil {
		return nil, m.mergeErr
	}

	// Basic mock merge: just return local for simplicity in tests
	return localVector, nil
}

func TestHybridCRDTMCP_Pull(t *testing.T) {
	provider := &MockCRDTProvider{
		pullData: map[string]json.RawMessage{
			"task-1": json.RawMessage(`{"clock": 1}`),
		},
	}
	mcp := NewHybridCRDTMCP(provider, true)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	// Test pull success
	res, err := mcp.CallTool(ctx, "crdt_pull", map[string]interface{}{"entity_id": "task-1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap := res.(map[string]interface{})
	if string(resMap["state_vector"].(json.RawMessage)) != `{"clock": 1}` {
		t.Errorf("expected state vector `{\"clock\": 1}`, got %s", resMap["state_vector"])
	}

	// Test multi-tenant enforcement
	ctxNoOrg := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{})
	_, err = mcp.CallTool(ctxNoOrg, "crdt_pull", map[string]interface{}{"entity_id": "task-1"})
	if err == nil || err.Error() != "unauthorized: missing claims or organization ID in multi-tenant mode" {
		t.Errorf("expected unauthorized error, got %v", err)
	}
}

func TestHybridCRDTMCP_Push(t *testing.T) {
	provider := &MockCRDTProvider{}
	mcp := NewHybridCRDTMCP(provider, false) // Standalone mode

	ctx := context.Background() // Missing claims should be tolerated in standalone

	// Test push success with map
	res, err := mcp.CallTool(ctx, "crdt_push", map[string]interface{}{
		"entity_id": "task-2",
		"state_vector": map[string]interface{}{"clock": 2},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success status, got %v", resMap["status"])
	}

	if string(provider.pullData["task-2"]) != `{"clock":2}` {
		t.Errorf("expected state vector `{\"clock\":2}`, got %s", provider.pullData["task-2"])
	}
}

func TestHybridCRDTMCP_Merge(t *testing.T) {
	provider := &MockCRDTProvider{}
	mcp := NewHybridCRDTMCP(provider, false)

	ctx := context.Background()

	// Test merge success
	res, err := mcp.CallTool(ctx, "crdt_merge", map[string]interface{}{
		"local_vector": map[string]interface{}{"clock": 3, "val": "A"},
		"remote_vector": map[string]interface{}{"clock": 4, "val": "B"},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap := res.(map[string]interface{})
	merged := resMap["merged_vector"].(json.RawMessage)
	if string(merged) != `{"clock":3,"val":"A"}` {
		t.Errorf("expected `{\"clock\":3,\"val\":\"A\"}`, got %s", merged)
	}
}
