package hybridfsmcp_server

import (
	"context"
	"encoding/json"
	"testing"
)

func TestIntegration(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewProvider(tempDir)
	ctx := context.Background()

	// Tools
	tools := provider.GetTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools")
	}

	// Write
	writeArgs, _ := json.Marshal(map[string]interface{}{"path": "test.txt", "data": "integration"})
	res, err := provider.Execute(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if res.Status != "success" {
		t.Errorf("Expected success, got %s", res.Status)
	}
}
