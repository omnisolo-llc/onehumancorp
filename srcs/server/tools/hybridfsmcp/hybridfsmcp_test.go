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
	baseDir := t.TempDir()
	provider := NewLocalFSProvider(baseDir)
	ctx := context.Background()

	// Write
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	// List
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("Expected 1 file 'test.txt', got %v", infos)
	}

	// Traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected error for path traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	baseDir := t.TempDir()
	provider := NewCloudFSProvider(baseDir)

	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Check underlying filesystem to ensure tenant isolation
	data, err := os.ReadFile(filepath.Join(baseDir, "tenant1", "test.txt"))
	if err != nil {
		t.Fatalf("Failed to read underlying file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test missing claims
	ctxMissing := context.Background()
	err = provider.WriteFile(ctxMissing, "test.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected error for missing claims, got nil")
	}
}

func TestMCPServer(t *testing.T) {
	baseDir := t.TempDir()
	provider := NewLocalFSProvider(baseDir)
	server := NewMCPServer(provider)
	ctx := context.Background()

	// write
	writeInput := `{"path": "test.txt", "data": "hello mcp"}`
	_, err := server.HandleToolCall(ctx, "write_file", json.RawMessage(writeInput))
	if err != nil {
		t.Fatalf("HandleToolCall write_file failed: %v", err)
	}

	// read
	readInput := `{"path": "test.txt"}`
	res, err := server.HandleToolCall(ctx, "read_file", json.RawMessage(readInput))
	if err != nil {
		t.Fatalf("HandleToolCall read_file failed: %v", err)
	}
	var readOut map[string]string
	json.Unmarshal(res, &readOut)
	if readOut["content"] != "hello mcp" {
		t.Errorf("Expected 'hello mcp', got '%s'", readOut["content"])
	}

	// list
	listInput := `{"path": "."}`
	res, err = server.HandleToolCall(ctx, "list_directory", json.RawMessage(listInput))
	if err != nil {
		t.Fatalf("HandleToolCall list_directory failed: %v", err)
	}
	var listOut map[string][]string
	json.Unmarshal(res, &listOut)
	if len(listOut["files"]) != 1 || listOut["files"][0] != "test.txt" {
		t.Errorf("Expected 1 file 'test.txt', got %v", listOut["files"])
	}
}
