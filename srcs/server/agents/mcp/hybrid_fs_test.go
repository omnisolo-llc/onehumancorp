package mcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := &LocalFSProvider{baseDir: tempDir}

	err := provider.WriteFile(context.Background(), "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	data, err := provider.ReadFile(context.Background(), "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got '%s'", string(data))
	}

	err = provider.WriteFile(context.Background(), "../escape.txt", []byte("hack"))
	if err == nil {
		t.Errorf("expected error on path escape")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := &CloudFSProvider{baseVolume: tempDir}

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Pre-create the tenant dir for the test
	tenantDir := filepath.Join(tempDir, "tenant-1")
	os.MkdirAll(tenantDir, 0755)

	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", string(data))
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := &LocalFSProvider{baseDir: tempDir}
	mcp := NewHybridFSMCP(provider)

	res := mcp.ExecuteTool(context.Background(), "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello mcp",
	})
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}

	res = mcp.ExecuteTool(context.Background(), "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if res.Status != "success" {
		t.Errorf("expected success, got %s", res.Status)
	}
	if string(res.ResultData) != "hello mcp" {
		t.Errorf("expected 'hello mcp', got '%s'", string(res.ResultData))
	}
}
