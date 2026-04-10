package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(data) != "hello world" {
		t.Fatalf("expected 'hello world', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("failed to list directory: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("unexpected entries: %v", entries)
	}

	// Test Path Validation (Absolute Path)
	err = provider.WriteFile(ctx, "/etc/passwd", []byte("hacked"))
	if err == nil {
		t.Fatalf("expected error for absolute path")
	}

	// Test Path Validation (Outside Workspace)
	err = provider.WriteFile(ctx, "../test.txt", []byte("hacked"))
	if err == nil {
		t.Fatalf("expected error for path outside workspace")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant1"})

	// Setup tenant dir manually for testing ListDir if needed, but WriteFile should create it

	// Test WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Verify file was written to tenant dir
	content, err := os.ReadFile(filepath.Join(tempDir, "tenant1", "test.txt"))
	if err != nil {
		t.Fatalf("failed to read file from disk: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Fatalf("expected 'hello cloud', got '%s'", string(content))
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Fatalf("expected 'hello cloud', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("failed to list directory: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("unexpected entries: %v", entries)
	}

	// Test No Tenant ID
	ctxNoAuth := context.Background()
	_, err = provider.ReadFile(ctxNoAuth, "test.txt")
	if err == nil {
		t.Fatalf("expected error for missing OrganizationID")
	}

	// Test Path Validation (Absolute Path)
	err = provider.WriteFile(ctx, "/etc/passwd", []byte("hacked"))
	if err == nil {
		t.Fatalf("expected error for absolute path")
	}

	// Test Path Validation (Outside Workspace)
	err = provider.WriteFile(ctx, "../test.txt", []byte("hacked"))
	if err == nil {
		t.Fatalf("expected error for path outside tenant workspace")
	}
}

func TestServer(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewServer(provider)
	ctx := context.Background()

	// Test write_file
	writeParams := `{"path":"test.txt","content":"hello server"}`
	res := server.HandleRequest(ctx, "write_file", json.RawMessage(writeParams))
	if res.Status != "success" {
		t.Fatalf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}

	// Test read_file
	readParams := `{"path":"test.txt"}`
	res = server.HandleRequest(ctx, "read_file", json.RawMessage(readParams))
	if res.Status != "success" {
		t.Fatalf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}
	if string(res.ResultData) != "hello server" {
			t.Fatalf("expected 'hello server', got '%s'", string(res.ResultData))
		}

	// Test list_directory
	listParams := `{"path":"."}`
	res = server.HandleRequest(ctx, "list_directory", json.RawMessage(listParams))
	if res.Status != "success" {
		t.Fatalf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}
	var entries []string
	json.Unmarshal(res.ResultData, &entries)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("unexpected entries: %v", entries)
	}

	// Test unknown tool
	res = server.HandleRequest(ctx, "unknown_tool", json.RawMessage(`{}`))
	if res.Status != "error" {
		t.Fatalf("expected error, got %s", res.Status)
	}
}
