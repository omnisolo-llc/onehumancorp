package hybridfs

import (
	"context"
	"testing"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_Errors(t *testing.T) {
	ctx := context.Background()

	provider, err := NewLocalFSProvider(t.TempDir())
	if err != nil {
		t.Fatal(err)
	}

	// Invalid target path
	_, err = provider.ReadFile(ctx, "\x00")
	if err == nil {
		t.Error("expected error for invalid target path")
	}

	err = provider.WriteFile(ctx, "\x00", []byte("data"))
	if err == nil {
		t.Error("expected error for invalid target path")
	}

	_, err = provider.ListDir(ctx, "\x00")
	if err == nil {
		t.Error("expected error for invalid target path")
	}

	// ListDir non-existent directory
	_, err = provider.ListDir(ctx, "nonexistent_dir")
	if err != ErrNotFound {
		t.Errorf("expected ErrNotFound, got %v", err)
	}

	// WriteFile to a path where parent is a file
	err = provider.WriteFile(ctx, "file.txt", []byte("data"))
	if err != nil {
		t.Fatal(err)
	}
	err = provider.WriteFile(ctx, "file.txt/subfile.txt", []byte("data"))
	if err == nil {
		t.Error("expected error when parent is a file")
	}
}

func TestCloudFSProvider_Errors(t *testing.T) {
	provider, err := NewCloudFSProvider(t.TempDir())
	if err != nil {
		t.Fatal(err)
	}

	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Invalid target path
	_, err = provider.ReadFile(ctx, "\x00")
	if err == nil {
		t.Error("expected error for invalid target path")
	}

	err = provider.WriteFile(ctx, "\x00", []byte("data"))
	if err == nil {
		t.Error("expected error for invalid target path")
	}

	_, err = provider.ListDir(ctx, "\x00")
	if err == nil {
		t.Error("expected error for invalid target path")
	}

	// ListDir non-existent directory
	_, err = provider.ListDir(ctx, "nonexistent_dir")
	if err != ErrNotFound {
		t.Errorf("expected ErrNotFound, got %v", err)
	}

	// WriteFile to a path where parent is a file
	err = provider.WriteFile(ctx, "file.txt", []byte("data"))
	if err != nil {
		t.Fatal(err)
	}
	err = provider.WriteFile(ctx, "file.txt/subfile.txt", []byte("data"))
	if err == nil {
		t.Error("expected error when parent is a file")
	}
}

func TestServer_Errors(t *testing.T) {
	provider, _ := NewLocalFSProvider(t.TempDir())
	server := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Missing path for read_file
	_, err := server.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil || !strings.Contains(err.Error(), "missing or invalid 'path'") {
		t.Errorf("expected missing path error, got %v", err)
	}

	// Read error from provider
	_, err = server.CallTool(ctx, "read_file", map[string]interface{}{"path": "nonexistent.txt"})
	if err == nil {
		t.Error("expected error from provider")
	}

	// Missing path for write_file
	_, err = server.CallTool(ctx, "write_file", map[string]interface{}{"content": "data"})
	if err == nil || !strings.Contains(err.Error(), "missing or invalid 'path'") {
		t.Errorf("expected missing path error, got %v", err)
	}

	// Missing content for write_file
	_, err = server.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt"})
	if err == nil || !strings.Contains(err.Error(), "missing 'content'") {
		t.Errorf("expected missing content error, got %v", err)
	}

	// Invalid content type for write_file
	_, err = server.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": 123})
	if err == nil || !strings.Contains(err.Error(), "invalid 'content' argument type") {
		t.Errorf("expected invalid content type error, got %v", err)
	}

	// Write error from provider
	_, err = server.CallTool(ctx, "write_file", map[string]interface{}{"path": "\x00", "content": "data"})
	if err == nil {
		t.Error("expected error from provider")
	}

	// List error from provider
	_, err = server.CallTool(ctx, "list_dir", map[string]interface{}{"path": "\x00"})
	if err == nil {
		t.Error("expected error from provider")
	}

	// Test []byte content write_file
	_, err = server.CallTool(ctx, "write_file", map[string]interface{}{"path": "byte.txt", "content": []byte("byte data")})
	if err != nil {
		t.Errorf("unexpected error for []byte content: %v", err)
	}
}
