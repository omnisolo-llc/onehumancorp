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
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	testContent := []byte("hello world")
	err = provider.WriteFile(ctx, "test.txt", testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("expected %q, got %q", string(testContent), string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 {
		t.Errorf("expected 1 entry, got %d", len(entries))
	}
	if entries[0].Name != "test.txt" {
		t.Errorf("expected test.txt, got %s", entries[0].Name)
	}

	// Test path escape
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Error("expected error for path escape, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create cloud provider: %v", err)
	}

	// Create context with claims
	orgID := "org-123"
	claims := &auth.Claims{OrganizationID: orgID}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	testContent := []byte("hello cloud")
	err = provider.WriteFile(ctx, "test.txt", testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it was written to the correct tenant directory
	expectedPath := filepath.Join(tempDir, "tenants", orgID, "test.txt")
	data, err := os.ReadFile(expectedPath)
	if err != nil {
		t.Fatalf("failed to read written file directly: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("expected %q, got %q", string(testContent), string(data))
	}

	// Test ReadFile
	data, err = provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("expected %q, got %q", string(testContent), string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 {
		t.Errorf("expected 1 entry, got %d", len(entries))
	}

	// Test path escape
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Error("expected error for path escape, got nil")
	}

	// Test unauthenticated context
	unauthCtx := context.Background()
	_, err = provider.ReadFile(unauthCtx, "test.txt")
	if err == nil {
		t.Error("expected error for unauthenticated context, got nil")
	}
}

func TestServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local provider: %v", err)
	}

	server := NewServer(provider)
	registry := NewRegistry()
	server.RegisterTools(registry)

	ctx := context.Background()

	// Test write_file tool
	writeArgs := []byte(`{"path": "test.txt", "content": "hello mcp"}`)
	writeResult, err := registry.ExecuteTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("ExecuteTool write_file failed: %v", err)
	}
	if writeResult.Status != "success" {
		t.Errorf("expected success, got %s: %s", writeResult.Status, string(writeResult.ResultData))
	}

	// Test read_file tool
	readArgs := []byte(`{"path": "test.txt"}`)
	readResult, err := registry.ExecuteTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("ExecuteTool read_file failed: %v", err)
	}
	if readResult.Status != "success" {
		t.Errorf("expected success, got %s: %s", readResult.Status, string(readResult.ResultData))
	}
	if string(readResult.ResultData) != "hello mcp" {
		t.Errorf("expected 'hello mcp', got %s", string(readResult.ResultData))
	}

	// Test list_directory tool
	listArgs := []byte(`{"path": "."}`)
	listResult, err := registry.ExecuteTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("ExecuteTool list_directory failed: %v", err)
	}
	if listResult.Status != "success" {
		t.Errorf("expected success, got %s: %s", listResult.Status, string(listResult.ResultData))
	}

	var entries []FileInfo
	if err := json.Unmarshal(listResult.ResultData, &entries); err != nil {
		t.Fatalf("failed to unmarshal list_directory result: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Errorf("unexpected entries: %+v", entries)
	}
}
