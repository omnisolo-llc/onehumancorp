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
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)

	ctx := context.Background()
	claims := &auth.Claims{} // Claims not strictly required for local

	// Test Write
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test Read
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got %s", string(data))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("ListDir unexpected output")
	}

	// Test Traversal
	_, err = provider.ReadFile(ctx, claims, "../outside.txt")
	if err == nil {
		t.Error("Expected traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewCloudFSProvider(tmpDir)

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}

	// Test Write
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify tenant isolation on disk
	if _, err := os.Stat(filepath.Join(tmpDir, "tenant1", "test.txt")); os.IsNotExist(err) {
		t.Errorf("File not created in tenant directory")
	}

	// Test Read
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", string(data))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("ListDir unexpected output")
	}

	// Test Cross-Tenant Access Attempt
	claims2 := &auth.Claims{OrganizationID: "tenant2"}
	_, err = provider.ReadFile(ctx, claims2, "../tenant1/test.txt")
	if err == nil {
		t.Error("Expected cross-tenant access error, got nil")
	}

	// Test Missing Tenant Context
	emptyClaims := &auth.Claims{}
	_, err = provider.ReadFile(ctx, emptyClaims, "test.txt")
	if err == nil {
		t.Error("Expected missing tenant error, got nil")
	}
}

func TestServerExecution(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)
	server := NewServer(provider)

	ctx := context.Background()
	claims := &auth.Claims{}

	writeReq := WriteFileRequest{
		Path:    "hello.txt",
		Content: "world",
	}
	writeInput, _ := json.Marshal(writeReq)

	res := server.ExecuteTool(ctx, claims, "write_file", writeInput)
	if !res.Success {
		t.Fatalf("write_file tool failed: %v", res.Error)
	}

	readReq := ReadFileRequest{
		Path: "hello.txt",
	}
	readInput, _ := json.Marshal(readReq)

	res = server.ExecuteTool(ctx, claims, "read_file", readInput)
	if !res.Success {
		t.Fatalf("read_file tool failed: %v", res.Error)
	}

	var readOut map[string]string
	json.Unmarshal(res.Data, &readOut)
	if readOut["content"] != "world" {
		t.Errorf("Expected 'world', got %s", readOut["content"])
	}

	listReq := ListDirRequest{
		Path: ".",
	}
	listInput, _ := json.Marshal(listReq)

	res = server.ExecuteTool(ctx, claims, "list_directory", listInput)
	if !res.Success {
		t.Fatalf("list_directory tool failed: %v", res.Error)
	}

	var listOut map[string][]string
	json.Unmarshal(res.Data, &listOut)
	if len(listOut["files"]) != 1 || listOut["files"][0] != "hello.txt" {
		t.Errorf("Expected ['hello.txt'], got %v", listOut["files"])
	}

	// Unknown tool
	res = server.ExecuteTool(ctx, claims, "unknown", []byte("{}"))
	if res.Success {
		t.Errorf("Expected unknown tool to fail")
	}
}

func TestFactory(t *testing.T) {
	// Test standalone (default)
	p1 := NewFileSystemProvider("/tmp/foo")
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider by default")
	}

	// Test cloud mode
	t.Setenv("OHC_MULTITENANT", "true")
	p2 := NewFileSystemProvider("/tmp/foo")
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider in multitenant mode")
	}
}
