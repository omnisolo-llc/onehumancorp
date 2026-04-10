package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	t.Run("Write and Read File", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello local"))
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		data, err := provider.ReadFile(ctx, nil, "test.txt")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if string(data) != "hello local" {
			t.Fatalf("expected 'hello local', got %s", string(data))
		}
	})

	t.Run("Path Traversal Blocked", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "../outside.txt", []byte("hack"))
		if err == nil {
			t.Fatal("expected error for path traversal, got nil")
		}
	})
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()

	claims := &auth.Claims{OrganizationID: "tenant1"}
	claims2 := &auth.Claims{OrganizationID: "tenant2"}

	t.Run("Write and Read File Tenant 1", func(t *testing.T) {
		err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello tenant1"))
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		data, err := provider.ReadFile(ctx, claims, "test.txt")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if string(data) != "hello tenant1" {
			t.Fatalf("expected 'hello tenant1', got %s", string(data))
		}
	})

	t.Run("Tenant Isolation", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, claims2, "test.txt")
		if err == nil {
			t.Fatal("expected error reading tenant1 file as tenant2, got nil")
		}
	})

	t.Run("Path Traversal Blocked", func(t *testing.T) {
		err := provider.WriteFile(ctx, claims, "../tenant2/hack.txt", []byte("hack"))
		if err == nil {
			t.Fatal("expected error for path traversal, got nil")
		}
	})

	t.Run("Missing Claims", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "test.txt", []byte("hack"))
		if err == nil {
			t.Fatal("expected error for missing claims, got nil")
		}
	})
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcpServer := NewHybridFSMCP(provider)
	ctx := context.Background()

	t.Run("ListTools", func(t *testing.T) {
		tools := mcpServer.ListTools()
		if len(tools) != 3 {
			t.Fatalf("expected 3 tools, got %d", len(tools))
		}
	})

	t.Run("CallTool WriteFile and ReadFile", func(t *testing.T) {
		writeArgs := map[string]interface{}{
			"path":    "mcp_test.txt",
			"content": "mcp content",
		}
		res, err := mcpServer.CallTool(ctx, "write_file", writeArgs)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if res.(map[string]interface{})["status"] != "success" {
			t.Fatalf("expected status success, got %v", res)
		}

		readArgs := map[string]interface{}{
			"path": "mcp_test.txt",
		}
		res, err = mcpServer.CallTool(ctx, "read_file", readArgs)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if res.(map[string]interface{})["content"] != "mcp content" {
			t.Fatalf("expected content 'mcp content', got %v", res)
		}
	})

	t.Run("CallTool ListDirectory", func(t *testing.T) {
		os.MkdirAll(filepath.Join(tempDir, "subdir"), 0755)
		listArgs := map[string]interface{}{
			"path": ".",
		}
		res, err := mcpServer.CallTool(ctx, "list_directory", listArgs)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		entries := res.(map[string]interface{})["entries"].([]string)
		found := false
		for _, e := range entries {
			if e == "mcp_test.txt" {
				found = true
			}
		}
		if !found {
			t.Fatalf("expected to find 'mcp_test.txt' in list directory result")
		}
	})
}
