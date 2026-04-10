package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()

	// Test unauthorized (no claims)
	_, err := provider.ReadFile(ctx, "test.txt")
	if err == nil || err.Error() != "unauthorized: missing or invalid claims" {
		t.Fatalf("expected unauthorized error, got %v", err)
	}

	// Test authorized
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctxWithAuth := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Write file via provider
	content := []byte("hello world")
	err = provider.WriteFile(ctxWithAuth, "test.txt", content)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify written to correct path
	expectedPath := filepath.Join(tempDir, "tenant-1", "test.txt")
	readContent, err := os.ReadFile(expectedPath)
	if err != nil {
		t.Fatalf("unexpected error reading file directly: %v", err)
	}
	if string(readContent) != string(content) {
		t.Fatalf("expected %s, got %s", content, readContent)
	}

	// Read file via provider
	readContentProvider, err := provider.ReadFile(ctxWithAuth, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(readContentProvider) != string(content) {
		t.Fatalf("expected %s, got %s", content, readContentProvider)
	}

	// Test boundary escape attempt
	err = provider.WriteFile(ctxWithAuth, "../tenant-2/test.txt", content)
	if err == nil {
		t.Fatal("expected error on boundary escape attempt")
	}
}

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Write file via provider
	content := []byte("hello local")
	err := provider.WriteFile(ctx, "local.txt", content)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify written to correct path
	expectedPath := filepath.Join(tempDir, "local.txt")
	readContent, err := os.ReadFile(expectedPath)
	if err != nil {
		t.Fatalf("unexpected error reading file directly: %v", err)
	}
	if string(readContent) != string(content) {
		t.Fatalf("expected %s, got %s", content, readContent)
	}

	// Read file via provider
	readContentProvider, err := provider.ReadFile(ctx, "local.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(readContentProvider) != string(content) {
		t.Fatalf("expected %s, got %s", content, readContentProvider)
	}

	// Test boundary escape attempt
	err = provider.WriteFile(ctx, "../escape.txt", content)
	if err == nil {
		t.Fatal("expected error on boundary escape attempt")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCPWithProvider(provider)
	ctx := context.Background()

	// Test tools listing
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	// Test write tool
	contentStr := base64.StdEncoding.EncodeToString([]byte("mcp test"))
	argsWrite := map[string]interface{}{
		"path":    "mcp.txt",
		"content": contentStr,
	}
	resWrite, err := mcp.CallTool(ctx, "write_file", argsWrite)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap, ok := resWrite.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Fatalf("expected success, got %v", resWrite)
	}

	// Test read tool
	argsRead := map[string]interface{}{
		"path": "mcp.txt",
	}
	resRead, err := mcp.CallTool(ctx, "read_file", argsRead)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resReadMap, ok := resRead.(map[string]interface{})
	if !ok || resReadMap["status"] != "success" || resReadMap["content"] != contentStr {
		t.Fatalf("unexpected read result: %v", resRead)
	}

	// Test list dir tool
	argsList := map[string]interface{}{
		"path": ".",
	}
	resList, err := mcp.CallTool(ctx, "list_directory", argsList)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resListMap, ok := resList.(map[string]interface{})
	if !ok || resListMap["status"] != "success" {
		t.Fatalf("unexpected list result: %v", resList)
	}
	entries, ok := resListMap["entries"].([]map[string]interface{})
	if !ok || len(entries) != 1 || entries[0]["name"] != "mcp.txt" {
		t.Fatalf("unexpected list entries: %v", entries)
	}
}
