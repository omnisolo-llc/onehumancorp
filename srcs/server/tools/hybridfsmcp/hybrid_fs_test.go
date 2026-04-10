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
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test Path Traversal Prevention
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Errorf("Expected error for path traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify tenant isolation
	if _, err := os.Stat(filepath.Join(tempDir, "tenant-1", "test.txt")); os.IsNotExist(err) {
		t.Fatalf("File not written to correct tenant directory")
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test Missing Context
	err = provider.WriteFile(context.Background(), "test.txt", []byte("no context"))
	if err == nil {
		t.Errorf("Expected error for missing claims, got nil")
	}

	// Test Path Traversal Prevention
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Errorf("Expected error for path traversal, got nil")
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	p := NewProvider("/tmp")
	if _, ok := p.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider, got %T", p)
	}

	os.Setenv("OHC_MULTITENANT", "false")
	p = NewProvider("/tmp")
	if _, ok := p.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider, got %T", p)
	}
}

func TestServer(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewServer(provider)
	ctx := context.Background()

	// Write
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Data: "server test"})
	_, err := server.WriteFile(ctx, writeArgs)
	if err != nil {
		t.Fatalf("Server WriteFile failed: %v", err)
	}

	// Read
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "test.txt"})
	res, err := server.ReadFile(ctx, readArgs)
	if err != nil {
		t.Fatalf("Server ReadFile failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "server test" {
		t.Errorf("Expected 'server test', got '%v'", resMap["content"])
	}

	// List
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res, err = server.ListDir(ctx, listArgs)
	if err != nil {
		t.Fatalf("Server ListDir failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}
}
