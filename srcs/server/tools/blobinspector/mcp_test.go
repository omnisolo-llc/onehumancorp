package blobinspector

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/storage"
)

func TestBlobInspectorMCP_ListTools(t *testing.T) {
	provider := storage.NewS3Provider("test-bucket")
	mcp := NewBlobInspectorMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}
}

func TestBlobInspectorMCP_LocalProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "blobtest")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	// Create a test file
	testFilePath := filepath.Join(dir, "test.txt")
	if err := os.WriteFile(testFilePath, []byte("hello"), 0644); err != nil {
		t.Fatalf("Failed to write test file: %v", err)
	}

	provider, err := storage.NewLocalProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create local provider: %v", err)
	}

	mcp := NewBlobInspectorMCP(provider)
	ctx := context.Background()

	// Test ReadBlobMetadata
	res, err := mcp.CallTool(ctx, "read_blob_metadata", map[string]interface{}{
		"key": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool failed: %v", err)
	}

	meta := res.(map[string]interface{})
	if meta["status"] != "success" || meta["mode"] != "standalone" || meta["key"] != "test.txt" || meta["size"].(int64) != 5 {
		t.Fatalf("Unexpected metadata response: %v", meta)
	}

	// Test GetBlobURL
	resURL, err := mcp.CallTool(ctx, "get_blob_url", map[string]interface{}{
		"key": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool failed: %v", err)
	}

	urlData := resURL.(map[string]interface{})
	if urlData["status"] != "success" || urlData["url"] == "" {
		t.Fatalf("Unexpected url response: %v", urlData)
	}

	// Test ListBlobs
	resList, err := mcp.CallTool(ctx, "list_blobs", map[string]interface{}{
		"prefix": "test",
	})
	if err != nil {
		t.Fatalf("CallTool failed: %v", err)
	}

	listData := resList.(map[string]interface{})
	results := listData["results"].([]map[string]interface{})
	if len(results) != 1 || results[0]["key"] != "test.txt" {
		t.Fatalf("Unexpected list response: %v", listData)
	}
	// Test ListBlobs missing prefix
	resListMissing, errLocalMiss := mcp.CallTool(ctx, "list_blobs", map[string]interface{}{})
	if errLocalMiss != nil {
		t.Fatalf("CallTool failed: %v", errLocalMiss)
	}
	if resListMissing.(map[string]interface{})["status"] != "success" {
		t.Fatalf("Unexpected missing prefix behavior")
	}
}

func TestBlobInspectorMCP_CloudProvider(t *testing.T) {
	provider := storage.NewS3Provider("test-bucket")
	mcp := NewBlobInspectorMCP(provider)
	ctx := context.Background()

	// Call without claims should fail
	_, err := mcp.CallTool(ctx, "read_blob_metadata", map[string]interface{}{
		"key": "test.txt",
	})
	if err == nil {
		t.Fatalf("Expected error when missing claims in cloud mode")
	}

	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-123",
	})

	res, err := mcp.CallTool(ctxWithClaims, "read_blob_metadata", map[string]interface{}{
		"key": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool failed: %v", err)
	}

	meta := res.(map[string]interface{})
	if meta["status"] != "success" || meta["mode"] != "cloud" || meta["key"] != "test.txt" {
		t.Fatalf("Unexpected metadata response: %v", meta)
	}

	resURL, err := mcp.CallTool(ctxWithClaims, "get_blob_url", map[string]interface{}{
		"key": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool failed: %v", err)
	}

	urlData := resURL.(map[string]interface{})
	if urlData["status"] != "success" || urlData["url"] == "" {
		t.Fatalf("Unexpected url response: %v", urlData)
	}

	// Test Cloud list blobs
	resListCloud, errCloudList := mcp.CallTool(ctxWithClaims, "list_blobs", map[string]interface{}{
		"prefix": "test",
	})
	if errCloudList != nil {
		t.Fatalf("CallTool failed: %v", errCloudList)
	}
	listDataCloud := resListCloud.(map[string]interface{})
	if listDataCloud["status"] != "success" || listDataCloud["mode"] != "cloud" {
		t.Fatalf("Unexpected list response: %v", listDataCloud)
	}

	// Test invalid tool
	_, errInv := mcp.CallTool(ctxWithClaims, "invalid_tool", map[string]interface{}{})
	if errInv == nil {
		t.Fatalf("Expected error for invalid tool")
	}

	// Test missing keys
	_, errMiss := mcp.CallTool(ctxWithClaims, "read_blob_metadata", map[string]interface{}{})
	if errMiss == nil {
		t.Fatalf("Expected error for missing key")
	}

	_, errMiss2 := mcp.CallTool(ctxWithClaims, "get_blob_url", map[string]interface{}{})
	if errMiss2 == nil {
		t.Fatalf("Expected error for missing key")
	}
}

// Dummy auth.Claims implementation is available via auth.Claims
