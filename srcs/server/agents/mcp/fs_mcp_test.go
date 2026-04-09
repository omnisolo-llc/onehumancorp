package mcp

import (
	"context"
	"encoding/json"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"os"
	"testing"
)

func TestLocalFSProvider(t *testing.T) {
	dir := t.TempDir()
	os.Setenv("OHC_WORKSPACE_DIR", dir)
	defer os.Unsetenv("OHC_WORKSPACE_DIR")

	provider := NewLocalFSProvider()
	err := provider.WriteFile(context.Background(), "test.txt", "hello")
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	content, err := provider.ReadFile(context.Background(), "test.txt")
	if err != nil || content != "hello" {
		t.Fatalf("ReadFile failed or wrong content: %v", err)
	}

	files, err := provider.ListDir(context.Background(), ".")
	if err != nil || len(files) != 1 || files[0] != "test.txt" {
		t.Fatalf("ListDir failed: %v %v", files, err)
	}

	_, err = provider.ReadFile(context.Background(), "../outside.txt")
	if err == nil {
		t.Fatal("Expected path traversal error")
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir := t.TempDir()
	os.Setenv("OHC_TENANT_PV_DIR", dir)
	defer os.Unsetenv("OHC_TENANT_PV_DIR")

	provider := NewCloudFSProvider()
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	err := provider.WriteFile(ctx, "test.txt", "hello")
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil || content != "hello" {
		t.Fatalf("ReadFile failed or wrong content: %v", err)
	}

	files, err := provider.ListDir(ctx, ".")
	if err != nil || len(files) != 1 || files[0] != "test.txt" {
		t.Fatalf("ListDir failed: %v %v", files, err)
	}

	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Fatal("Expected path traversal error")
	}

	_, err = provider.ReadFile(context.Background(), "test.txt")
	if err == nil {
		t.Fatal("Expected auth error")
	}
}

func TestFileSystemMCPProxy(t *testing.T) {
	dir := t.TempDir()
	os.Setenv("OHC_WORKSPACE_DIR", dir)
	defer os.Unsetenv("OHC_WORKSPACE_DIR")

	proxy := NewFileSystemMCPProxy()

	writePayload := []byte(`{"path": "foo.txt", "content": "bar"}`)
	res := proxy.HandleWriteFile(context.Background(), writePayload)
	if res.Status != "success" {
		t.Fatalf("HandleWriteFile failed: %s", string(res.ResultData))
	}

	readPayload := []byte(`{"path": "foo.txt"}`)
	res = proxy.HandleReadFile(context.Background(), readPayload)
	if res.Status != "success" {
		t.Fatalf("HandleReadFile failed: %s", string(res.ResultData))
	}

	var readResult map[string]string
	json.Unmarshal(res.ResultData, &readResult)
	if readResult["content"] != "bar" {
		t.Fatalf("Expected 'bar', got '%s'", readResult["content"])
	}
}
