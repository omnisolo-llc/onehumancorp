package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.Background()

	// Test WriteFile
	testData := []byte("hello local")
	err = provider.WriteFile(ctx, "test.txt", testData)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Test ReadFile
	readData, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(readData) != string(testData) {
		t.Fatalf("expected %s, got %s", string(testData), string(readData))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("unexpected entries: %v", entries)
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil || !strings.Contains(err.Error(), "path traversal denied") {
		t.Fatalf("expected path traversal error, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	// Inject claims
	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	testData := []byte("hello cloud")
	err = provider.WriteFile(ctx, "test.txt", testData)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Test ReadFile
	readData, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(readData) != string(testData) {
		t.Fatalf("expected %s, got %s", string(testData), string(readData))
	}

	// Verify it wrote to the tenant directory
	tenantFile := filepath.Join(tempDir, "tenant-123", "test.txt")
	if _, err := os.Stat(tenantFile); os.IsNotExist(err) {
		t.Fatalf("expected file in tenant dir: %s", tenantFile)
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("unexpected entries: %v", entries)
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil || !strings.Contains(err.Error(), "path traversal denied") {
		t.Fatalf("expected path traversal error, got %v", err)
	}

	// Test No Claims
	emptyCtx := context.Background()
	_, err = provider.ReadFile(emptyCtx, "test.txt")
	if err == nil || !strings.Contains(err.Error(), "missing tenant claims") {
		t.Fatalf("expected auth error, got %v", err)
	}
}

func TestServer(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	tempDir, err := os.MkdirTemp("", "server_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)
	os.Setenv("OHC_LOCAL_WORKSPACE", tempDir)
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_LOCAL_WORKSPACE")

	server, err := NewServer()
	if err != nil {
		t.Fatal(err)
	}

	tools := server.Tools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Write
	writeArgs := []byte(`{"path":"test.txt","data":"hello mcp"}`)
	resp, err := server.Call(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(resp) != `{"status":"success"}` {
		t.Fatalf("unexpected resp: %s", string(resp))
	}

	// Read
	readArgs := []byte(`{"path":"test.txt"}`)
	resp, err = server.Call(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(resp) != "hello mcp" {
		t.Fatalf("unexpected resp: %s", string(resp))
	}

	// List
	listArgs := []byte(`{"path":""}`)
	resp, err = server.Call(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if !strings.Contains(string(resp), "test.txt") {
		t.Fatalf("unexpected list resp: %s", string(resp))
	}
}
