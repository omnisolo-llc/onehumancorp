package hybridcrdtmcp

import (
	"context"
	"os"
	"testing"
	"reflect"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestToolHandlers(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridCRDTMCP()

	t.Run("Pull with Claims", func(t *testing.T) {
		ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})
		res, err := mcp.CallTool(ctx, "crdt_pull", map[string]interface{}{"entity_id": "test-entity"})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if res == nil {
			t.Fatalf("expected response, got nil")
		}
	})

	t.Run("Pull without Claims", func(t *testing.T) {
		ctx := context.Background()
		_, err := mcp.CallTool(ctx, "crdt_pull", map[string]interface{}{"entity_id": "test-entity"})
		if err == nil {
			t.Fatalf("expected error without claims in multi-tenant mode, got nil")
		}
	})

	t.Run("Push and Pull with Claims", func(t *testing.T) {
		ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})
		_, err := mcp.CallTool(ctx, "crdt_push", map[string]interface{}{
			"entity_id": "test-entity",
			"vector": map[string]interface{}{"clock": 10.0},
		})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		res, err := mcp.CallTool(ctx, "crdt_pull", map[string]interface{}{"entity_id": "test-entity"})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		resMap, ok := res.(map[string]interface{})
		if !ok || resMap["status"] != "pulled" {
			t.Fatalf("expected pulled status")
		}

		vec, ok := resMap["vector"].(map[string]interface{})
		if !ok || vec["clock"] != 10.0 {
			t.Fatalf("expected clock to be 10, got %v", vec["clock"])
		}
	})

	t.Run("Merge", func(t *testing.T) {
		ctx := context.Background()
		res, err := mcp.CallTool(ctx, "crdt_merge", map[string]interface{}{
			"local_vector": map[string]interface{}{"clock": 5.0, "other": 2.0},
			"remote_vector": map[string]interface{}{"clock": 10.0, "new": 1.0},
		})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		resMap, ok := res.(map[string]interface{})
		if !ok {
			t.Fatalf("expected map response")
		}

		expected := map[string]interface{}{"clock": 10.0, "other": 2.0, "new": 1.0}
		if !reflect.DeepEqual(resMap["vector"], expected) {
			t.Fatalf("expected %v, got %v", expected, resMap["vector"])
		}
	})
}
