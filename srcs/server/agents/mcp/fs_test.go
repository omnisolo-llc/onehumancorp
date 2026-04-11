package mcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local fs provider: %v", err)
	}

	ctx := context.Background()

	// Valid path
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("expected success for valid path, got %v", err)
	}

	// Invalid path traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error for path traversal escaping base dir")
	}

	// Invalid path partial match check
	// Suppose baseDir is "/tmp/foo" and target is "../../foobar"
	// filepath.Clean might return "/tmp/foobar"
	// We want to ensure strings.HasPrefix correctly rejects "/tmp/foobar" when base is "/tmp/foo/"
}

func TestCloudFSProvider_TenantIsolation(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create cloud fs provider: %v", err)
	}

	// Setup context with claims
	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Valid path
	err = provider.WriteFile(ctx, "data.txt", []byte("cloud"))
	if err != nil {
		t.Errorf("expected success for valid path, got %v", err)
	}

	// Verify file is actually in the tenant scoped directory
	expectedPath := filepath.Join(tempDir, "tenant-tenant1", "data.txt")
	content, err := os.ReadFile(expectedPath)
	if err != nil {
		t.Errorf("expected file to be created at %s, got %v", expectedPath, err)
	}
	if string(content) != "cloud" {
		t.Errorf("expected file content 'cloud', got %s", string(content))
	}

	// Invalid path traversal
	err = provider.WriteFile(ctx, "../tenant2/data.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error for path traversal escaping tenant dir")
	}

	// Test missing organization ID
	ctxNoAuth := context.Background()
	err = provider.WriteFile(ctxNoAuth, "test.txt", []byte("test"))
	if err == nil {
		t.Error("expected error when context has no claims")
	}
}

func TestFSServer_Tools(t *testing.T) {
	tempDir := t.TempDir()
	provider, _ := NewLocalFSProvider(tempDir)
	server := NewFSServer(provider)

	ctx := context.Background()

	// Write file tool
	writeInput, _ := json.Marshal(map[string]string{
		"path":    "hello.txt",
		"content": "world",
	})

	res, err := server.WriteFileTool(ctx, writeInput)
	if err != nil {
		t.Fatalf("WriteFileTool failed: %v", err)
	}
	if res.Status != "success" {
		t.Errorf("expected status 'success', got %s", res.Status)
	}

	// Read file tool
	readInput, _ := json.Marshal(map[string]string{
		"path": "hello.txt",
	})

	res, err = server.ReadFileTool(ctx, readInput)
	if err != nil {
		t.Fatalf("ReadFileTool failed: %v", err)
	}

	var readResult map[string]string
	json.Unmarshal(res.ResultData, &readResult)
	if readResult["content"] != "world" {
		t.Errorf("expected content 'world', got %s", readResult["content"])
	}

	// List dir tool
	listInput, _ := json.Marshal(map[string]string{
		"path": ".",
	})

	res, err = server.ListDirTool(ctx, listInput)
	if err != nil {
		t.Fatalf("ListDirTool failed: %v", err)
	}

	var listResult map[string]interface{}
	json.Unmarshal(res.ResultData, &listResult)

	entries := listResult["entries"].([]interface{})
	if len(entries) != 1 || entries[0].(string) != "hello.txt" {
		t.Errorf("expected [hello.txt], got %v", entries)
	}
}

func TestNewFileSystemProviderFactory(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	p, err := NewFileSystemProvider("/tmp")
	if err != nil {
		t.Fatalf("factory failed: %v", err)
	}
	if _, ok := p.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider when OHC_STANDALONE=true")
	}

	t.Setenv("OHC_STANDALONE", "false")
	p2, err := NewFileSystemProvider("/tmp")
	if err != nil {
		t.Fatalf("factory failed: %v", err)
	}
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider when OHC_STANDALONE=false")
	}
}
