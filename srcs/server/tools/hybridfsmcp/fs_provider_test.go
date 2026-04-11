package hybridfsmcp

import (
	"context"
	"os"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	baseDir := t.TempDir()
	provider := NewLocalFSProvider(baseDir)
	ctx := context.Background()

	// Write inside
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error writing file: %v", err)
	}

	// Read inside
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error reading file: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got %s", string(data))
	}

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error listing dir: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Fatalf("expected one entry 'test.txt', got %v", entries)
	}

	// Traversal Attempt
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil || !strings.Contains(err.Error(), "path traversal detected") {
		t.Fatalf("expected path traversal error, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	baseMountPoint := t.TempDir()
	provider := NewCloudFSProvider(baseMountPoint)

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write inside
	err := provider.WriteFile(ctx, "data.json", []byte(`{"key":"value"}`))
	if err != nil {
		t.Fatalf("unexpected error writing file: %v", err)
	}

	// Read inside
	data, err := provider.ReadFile(ctx, "data.json")
	if err != nil {
		t.Fatalf("unexpected error reading file: %v", err)
	}
	if string(data) != `{"key":"value"}` {
		t.Fatalf("expected json, got %s", string(data))
	}

	// Traversal Attempt
	_, err = provider.ReadFile(ctx, "../tenant-2/data.json")
	if err == nil || !strings.Contains(err.Error(), "path traversal detected") {
		t.Fatalf("expected path traversal error, got %v", err)
	}

	// Unauthenticated
	ctxUnauth := context.Background()
	_, err = provider.ReadFile(ctxUnauth, "data.json")
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Fatalf("expected unauthorized error, got %v", err)
	}
}

func TestServerFactoryAndHandle(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	server := NewServer(nil)

	_, ok := server.Provider.(*LocalFSProvider)
	if !ok {
		t.Fatalf("expected LocalFSProvider when OHC_MULTITENANT is false")
	}

	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_FS_ROOT", t.TempDir())
	server = NewServer(nil)

	_, ok = server.Provider.(*CloudFSProvider)
	if !ok {
		t.Fatalf("expected CloudFSProvider when OHC_MULTITENANT is true")
	}

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant-1"})

	writeResult := server.HandleToolCall(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"content": "hello world",
	})
	if writeResult.Status != "success" {
		t.Fatalf("expected write success, got %v: %s", writeResult.Status, string(writeResult.ResultData))
	}

	readResult := server.HandleToolCall(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if readResult.Status != "success" || string(readResult.ResultData) != "hello world" {
		t.Fatalf("expected read success 'hello world', got %v: %s", readResult.Status, string(readResult.ResultData))
	}

	listResult := server.HandleToolCall(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if listResult.Status != "success" || !strings.Contains(string(listResult.ResultData), "test.txt") {
		t.Fatalf("expected list success with test.txt, got %v: %s", listResult.Status, string(listResult.ResultData))
	}
}
