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
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("expected 1 file 'test.txt', got %v", infos)
	}

	// Test Boundary constraints
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("expected error when escaping base dir, got nil")
	}

	// Test Boundary constraints using a suffix directory trick
	err = provider.WriteFile(ctx, "../" + filepath.Base(tempDir) + "_suffix/outside.txt", []byte("hello"))
	if err == nil {
		t.Errorf("expected error when escaping base dir with suffix trick, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	// For tests we use the fallback context key so we don't have to duplicate the auth middleware's unexported key logic.
	ctxWithClaims := context.WithValue(context.Background(), "auth_claims_test_fallback", &auth.Claims{
		OrganizationID: "tenant1",
	})

	ctxWithoutClaims := context.Background()

	// Test WriteFile with claims
	err = provider.WriteFile(ctxWithClaims, "data.txt", []byte("cloud"))
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Test ReadFile with claims
	data, err := provider.ReadFile(ctxWithClaims, "data.txt")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if string(data) != "cloud" {
		t.Errorf("expected 'cloud', got '%s'", string(data))
	}

	// Test WriteFile without claims
	err = provider.WriteFile(ctxWithoutClaims, "data.txt", []byte("cloud"))
	if err == nil {
		t.Errorf("expected error without claims, got nil")
	}

	// Test Boundary constraints
	_, err = provider.ReadFile(ctxWithClaims, "../outside.txt")
	if err == nil {
		t.Errorf("expected error when escaping tenant dir, got nil")
	}

	// Test Boundary constraints using a suffix directory trick
	err = provider.WriteFile(ctxWithClaims, "../tenant1_suffix/outside.txt", []byte("hello"))
	if err == nil {
		t.Errorf("expected error when escaping base dir with suffix trick, got nil")
	}

	// Verify tenant isolation visually
	tenantDir := filepath.Join(tempDir, "tenant1")
	if _, err := os.Stat(tenantDir); os.IsNotExist(err) {
		t.Errorf("tenant directory was not created: %v", err)
	}
}

func TestFactory(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	p1, _ := NewFileSystemProvider(".")
	if _, ok := p1.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider when OHC_MULTITENANT=true")
	}

	t.Setenv("OHC_MULTITENANT", "false")
	p2, _ := NewFileSystemProvider(".")
	if _, ok := p2.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider when OHC_MULTITENANT=false")
	}
}

func TestMCPFSHandler(t *testing.T) {
	tempDir := t.TempDir()
	provider, _ := NewLocalFSProvider(tempDir)
	handler := NewMCPFSHandler(provider)
	ctx := context.Background()

	// Write Tool
	writeArgs := WriteFileArgs{Path: "test.txt", Data: "mcp_data"}
	writeRaw, _ := json.Marshal(writeArgs)
	res := handler.Handle(ctx, "write_file", writeRaw)
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}

	// Read Tool
	readArgs := ReadFileArgs{Path: "test.txt"}
	readRaw, _ := json.Marshal(readArgs)
	res = handler.Handle(ctx, "read_file", readRaw)
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}

	var readRes map[string]string
	json.Unmarshal(res.ResultData, &readRes)
	if readRes["content"] != "mcp_data" {
		t.Errorf("expected 'mcp_data', got %v", readRes["content"])
	}

	// List Tool
	listArgs := ListDirArgs{Path: "."}
	listRaw, _ := json.Marshal(listArgs)
	res = handler.Handle(ctx, "list_directory", listRaw)
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, string(res.ResultData))
	}

	// Unknown Tool
	res = handler.Handle(ctx, "unknown_tool", []byte("{}"))
	if res.Status != "error" {
		t.Errorf("expected error for unknown tool, got %s", res.Status)
	}
}
