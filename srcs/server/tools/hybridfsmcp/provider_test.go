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

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected hello, got %s", string(data))
	}

	// Test boundary logic
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hacker"))
	if err == nil {
		t.Errorf("expected error for path traversal")
	}

	err = provider.WriteFile(ctx, "/etc/passwd", []byte("hacker"))
	if err == nil {
		t.Errorf("expected error for absolute path")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected hello cloud, got %s", string(data))
	}

	// Read raw from FS to verify tenant directory creation
	raw, err := os.ReadFile(filepath.Join(tempDir, "tenant-1", "test.txt"))
	if err != nil {
		t.Fatalf("file not created in tenant dir: %v", err)
	}
	if string(raw) != "hello cloud" {
		t.Errorf("expected hello cloud, got %s", string(raw))
	}
}

func TestMCPServer(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewFileSystemMCPServer(provider)
	ctx := context.Background()

	writeArgs := map[string]interface{}{"path": "test.txt", "content": "mcp hello"}
	_, err := server.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	readArgs := map[string]interface{}{"path": "test.txt"}
	res, err := server.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	m := res.(map[string]interface{})
	if m["content"] != "mcp hello" {
		t.Errorf("expected mcp hello, got %s", m["content"])
	}
}
