package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got '%s'", string(data))
	}

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("expected 1 entry 'test.txt', got %v", entries)
	}

	// Path traversal
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Fatal("expected path traversal error")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("cloud"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify it wrote to tenant dir
	content, err := os.ReadFile(filepath.Join(tempDir, "tenant-1", "test.txt"))
	if err != nil {
		t.Fatalf("unexpected error reading raw file: %v", err)
	}
	if string(content) != "cloud" {
		t.Fatalf("expected 'cloud', got '%s'", string(content))
	}

	// Read file via provider
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "cloud" {
		t.Fatalf("expected 'cloud', got '%s'", string(data))
	}

	// No claims context should fail
	_, err = provider.ReadFile(context.Background(), "test.txt")
	if err == nil {
		t.Fatal("expected error due to missing claims")
	}
}

func TestMCPCallTool(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSInspectorMCP(provider)
	ctx := context.Background()

	// Write file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":        "mcp.txt",
		"content_b64": base64.StdEncoding.EncodeToString([]byte("mcp-test")),
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Read file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content_b64"] != base64.StdEncoding.EncodeToString([]byte("mcp-test")) {
		t.Fatalf("unexpected result: %v", resMap)
	}

	// List dir
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]map[string]interface{})
	if len(entries) != 1 || entries[0]["name"] != "mcp.txt" {
		t.Fatalf("unexpected entries: %v", entries)
	}
}
