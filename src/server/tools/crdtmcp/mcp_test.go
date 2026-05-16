package crdtmcp

import (
	"context"
	"testing"
	"time"
)

type mockProvider struct {
	pulled bool
	pushed []CrdtDelta
}

func (m *mockProvider) Pull(ctx context.Context) ([]CrdtDelta, error) {
	m.pulled = true
	return []CrdtDelta{
		{ID: "1", EntityID: "e1", Data: "{}", UpdatedAt: time.Now()},
	}, nil
}

func (m *mockProvider) Push(ctx context.Context, deltas []CrdtDelta) error {
	m.pushed = append(m.pushed, deltas...)
	return nil
}

func TestCrdtMCP(t *testing.T) {
	ctx := context.Background()
	provider := &mockProvider{}
	mcp := NewCrdtMCP(provider)

	res, err := mcp.CallTool(ctx, "crdt_pull", map[string]interface{}{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !provider.pulled {
		t.Errorf("expected provider to be called for pull")
	}

	mRes, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", res)
	}

	if _, ok := mRes["deltas"]; !ok {
		t.Errorf("expected deltas in response")
	}

	args := map[string]interface{}{
		"deltas": []interface{}{
			map[string]interface{}{
				"id":         "d1",
				"entity_id":  "e1",
				"data":       `{"foo":"bar"}`,
				"updated_at": "2026-04-17T12:00:00Z",
			},
		},
	}

	res, err = mcp.CallTool(ctx, "crdt_push", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(provider.pushed) != 1 {
		t.Errorf("expected 1 push, got %d", len(provider.pushed))
	}

	if provider.pushed[0].ID != "d1" {
		t.Errorf("expected pushed ID to be 'd1', got %s", provider.pushed[0].ID)
	}

	tools := mcp.ListTools()
	if len(tools) != 2 {
		t.Errorf("expected 2 tools, got %d", len(tools))
	}
}
