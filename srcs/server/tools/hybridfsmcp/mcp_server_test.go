package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs-test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	// Test list
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("Unexpected list result: %v", infos)
	}

	// Test list empty directory
	emptyDir := filepath.Join(tempDir, "empty")
	os.MkdirAll(emptyDir, 0755)
	emptyInfos, err := provider.ListDir(ctx, "empty")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(emptyInfos) != 0 {
		t.Errorf("Expected empty dir, got %v", emptyInfos)
	}

	// Test bounds
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hello"))
	if err == nil {
		t.Error("Expected error when escaping bounds")
	}

	// Test resolve path errors
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Error("Expected error reading escaped path")
	}
	_, err = provider.ListDir(ctx, "../escape.txt")
	if err == nil {
		t.Error("Expected error listing escaped path")
	}

	// Test prefix attack
	// If base path is /tmp/localfs-test
	// Attack path is /tmp/localfs-test-attack
	attackDir := tempDir + "-attack"
	os.MkdirAll(attackDir, 0755)
	defer os.RemoveAll(attackDir)

	attackFile := filepath.Join(attackDir, "secret.txt")
	os.WriteFile(attackFile, []byte("secret"), 0644)

	// Try to read attack file using absolute path traversing up
	relAttack, _ := filepath.Rel(tempDir, attackFile)
	_, err = provider.ReadFile(ctx, relAttack)
	if err == nil {
		t.Error("Expected error when accessing prefix attack path")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs-test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	// Test unauthorized write/read/list
	unauthCtx := context.Background()
	_, err = provider.ReadFile(unauthCtx, "test.txt")
	if err == nil {
		t.Error("Expected error without claims (ReadFile)")
	}
	err = provider.WriteFile(unauthCtx, "test.txt", []byte("test"))
	if err == nil {
		t.Error("Expected error without claims (WriteFile)")
	}
	_, err = provider.ListDir(unauthCtx, "test.txt")
	if err == nil {
		t.Error("Expected error without claims (ListDir)")
	}

	// Test blank tenantID
	blankClaims := &auth.Claims{OrganizationID: ""}
	blankCtx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, blankClaims)
	_, err = provider.ReadFile(blankCtx, "test.txt")
	if err == nil {
		t.Error("Expected error with blank tenant ID")
	}

	// Ensure tenant separation
	tenantPath := filepath.Join(tempDir, "tenant-1", "test.txt")
	if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
		t.Error("File not created in tenant-specific path")
	}

	// Test bounds
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hello"))
	if err == nil {
		t.Error("Expected error when escaping bounds")
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("Unexpected list result: %v", infos)
	}

	// Prefix attack test
	tenant10Claims := &auth.Claims{OrganizationID: "tenant-10"}
	tenant10Ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, tenant10Claims)
	provider.WriteFile(tenant10Ctx, "secret.txt", []byte("secret"))

	_, err = provider.ReadFile(ctx, "../tenant-10/secret.txt")
	if err == nil {
		t.Error("Expected error accessing other tenant data using prefix attack")
	}
}

func TestFileSystemMCPServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp-test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	server := NewFileSystemMCPServer(provider)
	ctx := context.Background()

	// Test write tool
	writeArgsStr := `{"path": "test.txt", "data": "hello"}` // "hello" in plain text since it's a string
	res, err := server.ExecuteTool(ctx, "write_file", []byte(writeArgsStr))
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	if string(res.ResultData) != `{"status": "ok"}` {
		t.Errorf("Unexpected write result: %s", string(res.ResultData))
	}

	// Test write error
	_, err = server.ExecuteTool(ctx, "write_file", []byte(`{"invalid json`))
	if err == nil {
		t.Errorf("Expected error with invalid JSON for write_file")
	}

	// Test read tool
	readArgs := []byte(`{"path": "test.txt"}`)
	res, err = server.ExecuteTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}

	var unmarshaledReadResult string
	err = json.Unmarshal(res.ResultData, &unmarshaledReadResult)
	if err != nil {
		t.Fatalf("failed to unmarshal read result: %v", err)
	}
	if unmarshaledReadResult != "hello" {
		t.Errorf("Expected 'hello', got %s", unmarshaledReadResult)
	}

	// Test read error
	_, err = server.ExecuteTool(ctx, "read_file", []byte(`{"invalid json`))
	if err == nil {
		t.Errorf("Expected error with invalid JSON for read_file")
	}

	// Test list_directory tool
	listArgs := []byte(`{"path": "."}`)
	res, err = server.ExecuteTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("list_directory tool failed: %v", err)
	}

	var unmarshaledListResult []map[string]interface{}
	err = json.Unmarshal(res.ResultData, &unmarshaledListResult)
	if err != nil {
		t.Fatalf("failed to unmarshal list result: %v", err)
	}
	if len(unmarshaledListResult) != 1 || unmarshaledListResult[0]["name"] != "test.txt" {
		t.Errorf("Unexpected list result: %v", unmarshaledListResult)
	}

	// Test list_directory empty results
	emptyDir := filepath.Join(tempDir, "empty")
	os.MkdirAll(emptyDir, 0755)
	emptyListArgs := []byte(`{"path": "empty"}`)
	res, err = server.ExecuteTool(ctx, "list_directory", emptyListArgs)
	if err != nil {
		t.Fatalf("list_directory tool failed: %v", err)
	}

	err = json.Unmarshal(res.ResultData, &unmarshaledListResult)
	if err != nil {
		t.Fatalf("failed to unmarshal empty list result: %v", err)
	}
	if len(unmarshaledListResult) != 0 {
		t.Errorf("Expected empty list, got: %v", unmarshaledListResult)
	}

	// Test list_directory error
	_, err = server.ExecuteTool(ctx, "list_directory", []byte(`{"invalid json`))
	if err == nil {
		t.Errorf("Expected error with invalid JSON for list_directory")
	}

	// Test unknown tool
	_, err = server.ExecuteTool(ctx, "unknown_tool", []byte(`{}`))
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}
}

func TestFactory(t *testing.T) {
	// Test Cloud
	os.Setenv("OHC_MULTITENANT", "true")
	provider := NewFileSystemProvider("/tmp")
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider")
	}

	// Test Local
	os.Setenv("OHC_MULTITENANT", "false")
	provider = NewFileSystemProvider("/tmp")
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider")
	}
}
