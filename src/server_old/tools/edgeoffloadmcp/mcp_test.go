package edgeoffloadmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/src/server/auth"
)

func TestListTools(t *testing.T) {
	router := NewRouter()
	tools := router.ListTools()
	if len(tools) != 1 {
		t.Fatalf("expected 1 tool, got %d", len(tools))
	}
	if tools[0].Name != "mcp_inference_router" {
		t.Errorf("expected tool name mcp_inference_router, got %s", tools[0].Name)
	}
}

func TestCallTool_UnknownTool(t *testing.T) {
	router := NewRouter()
	_, err := router.CallTool(context.Background(), "unknown", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error for unknown tool")
	}
}

func TestCallTool_MissingArguments(t *testing.T) {
	router := NewRouter()

	// Missing prompt
	_, err := router.CallTool(context.Background(), "mcp_inference_router", map[string]interface{}{
		"is_sensitive": true,
		"complexity": "low",
	})
	if err == nil {
		t.Fatal("expected error for missing prompt")
	}

	// Missing is_sensitive
	_, err = router.CallTool(context.Background(), "mcp_inference_router", map[string]interface{}{
		"prompt": "test",
		"complexity": "low",
	})
	if err == nil {
		t.Fatal("expected error for missing is_sensitive")
	}

	// Missing complexity
	_, err = router.CallTool(context.Background(), "mcp_inference_router", map[string]interface{}{
		"prompt": "test",
		"is_sensitive": true,
	})
	if err == nil {
		t.Fatal("expected error for missing complexity")
	}
}

func TestCallTool_LocalRoute(t *testing.T) {
	router := NewRouter()

	// Sensitive
	res, err := router.CallTool(context.Background(), "mcp_inference_router", map[string]interface{}{
		"prompt": "test",
		"is_sensitive": true,
		"complexity": "high",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["route"] != "local" {
		t.Errorf("expected route local, got %s", resMap["route"])
	}
	if resMap["status"] != "success" {
		t.Errorf("expected status success, got %s", resMap["status"])
	}

	// Low complexity
	res, err = router.CallTool(context.Background(), "mcp_inference_router", map[string]interface{}{
		"prompt": "test",
		"is_sensitive": false,
		"complexity": "low",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["route"] != "local" {
		t.Errorf("expected route local, got %s", resMap["route"])
	}
}

func TestCallTool_CloudRoute(t *testing.T) {
	router := NewRouter()

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	res, err := router.CallTool(ctx, "mcp_inference_router", map[string]interface{}{
		"prompt": "test",
		"is_sensitive": false,
		"complexity": "high",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["route"] != "cloud" {
		t.Errorf("expected route cloud, got %s", resMap["route"])
	}
	if resMap["status"] != "success" {
		t.Errorf("expected status success, got %s", resMap["status"])
	}
}

func TestCallTool_Fallback_Unauthorized(t *testing.T) {
	router := NewRouter()

	// No claims in context
	ctx := context.Background()

	res, err := router.CallTool(ctx, "mcp_inference_router", map[string]interface{}{
		"prompt": "test",
		"is_sensitive": false,
		"complexity": "high",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["route"] != "local" {
		t.Errorf("expected route local, got %s", resMap["route"])
	}
	if resMap["status"] != "fallback" {
		t.Errorf("expected status fallback, got %s", resMap["status"])
	}
}

func TestCallTool_Fallback_CloudError(t *testing.T) {
	router := NewRouter()

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	os.Setenv("MOCK_CLOUD_ERROR", "true")
	defer os.Unsetenv("MOCK_CLOUD_ERROR")

	res, err := router.CallTool(ctx, "mcp_inference_router", map[string]interface{}{
		"prompt": "test",
		"is_sensitive": false,
		"complexity": "high",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["route"] != "local" {
		t.Errorf("expected route local, got %s", resMap["route"])
	}
	if resMap["status"] != "fallback" {
		t.Errorf("expected status fallback, got %s", resMap["status"])
	}
}
