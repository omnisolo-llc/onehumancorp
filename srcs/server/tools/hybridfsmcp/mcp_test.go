package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello world" {
		t.Fatalf("Expected 'hello world', got '%s'", string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("Expected ['test.txt'], got %v", entries)
	}

	err = provider.WriteFile(ctx, "../outside.txt", []byte("hacker"))
	if err == nil {
		t.Fatalf("Expected error when writing outside base dir")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctx, "test.txt", []byte("hello tenant"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello tenant" {
		t.Fatalf("Expected 'hello tenant', got '%s'", string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("Expected ['test.txt'], got %v", entries)
	}

	err = provider.WriteFile(ctx, "../outside.txt", []byte("hacker"))
	if err == nil {
		t.Fatalf("Expected error when writing outside tenant dir")
	}

	// Ensure tenant separation
	claims2 := &auth.Claims{OrganizationID: "tenant2"}
	ctx2 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims2)

	_, err = provider.ReadFile(ctx2, "test.txt")
	if err == nil {
		t.Fatalf("Tenant 2 should not be able to read tenant 1's file")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "hybridfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCPWithProvider(provider)
	ctx := context.Background()

	// Test write
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp hello",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Test read
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "mcp hello" {
		t.Fatalf("Expected 'mcp hello', got '%v'", resMap["content"])
	}

	// Test list
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "mcp_test.txt" {
		t.Fatalf("Expected ['mcp_test.txt'], got %v", entries)
	}
}
