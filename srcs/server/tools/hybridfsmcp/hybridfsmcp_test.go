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

	t.Run("Write and Read File", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test.txt", []byte("hello world"))
		if err != nil {
			t.Fatalf("WriteFile failed: %v", err)
		}

		data, err := provider.ReadFile(ctx, "test.txt")
		if err != nil {
			t.Fatalf("ReadFile failed: %v", err)
		}

		if string(data) != "hello world" {
			t.Errorf("expected 'hello world', got '%s'", string(data))
		}
	})

	t.Run("List Directory", func(t *testing.T) {
		infos, err := provider.ListDir(ctx, ".")
		if err != nil {
			t.Fatalf("ListDir failed: %v", err)
		}
		if len(infos) != 1 || infos[0].Name() != "test.txt" {
			t.Errorf("expected 1 file 'test.txt', got %v", infos)
		}
	})

	t.Run("Path Traversal", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "../../../etc/passwd")
		if err == nil {
			t.Error("expected error for path traversal, got nil")
		}
	})

	t.Run("Absolute Path", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "/etc/passwd")
		if err == nil {
			t.Error("expected error for absolute path, got nil")
		}
	})
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

	// Mock auth context
	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	t.Run("Write and Read File Tenant Scoped", func(t *testing.T) {
		err := provider.WriteFile(ctx, "test.txt", []byte("tenant data"))
		if err != nil {
			t.Fatalf("WriteFile failed: %v", err)
		}

		data, err := provider.ReadFile(ctx, "test.txt")
		if err != nil {
			t.Fatalf("ReadFile failed: %v", err)
		}

		if string(data) != "tenant data" {
			t.Errorf("expected 'tenant data', got '%s'", string(data))
		}

		// Verify it was written to the correct tenant dir
		tenantPath := filepath.Join(tempDir, "tenant-123", "test.txt")
		if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
			t.Errorf("expected file to exist at %s", tenantPath)
		}
	})

	t.Run("Missing Organization ID", func(t *testing.T) {
		emptyCtx := context.Background()
		_, err := provider.ReadFile(emptyCtx, "test.txt")
		if err == nil {
			t.Error("expected error for missing organization ID, got nil")
		}
	})

	t.Run("Path Traversal Escape", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, "../tenant-456/test.txt")
		if err == nil {
			t.Error("expected error for path traversal, got nil")
		}
	})
}

func TestServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	server := NewServer(provider)

	ctx := context.Background()

	t.Run("GetTools", func(t *testing.T) {
		tools := server.GetTools()
		if len(tools) != 3 {
			t.Errorf("expected 3 tools, got %d", len(tools))
		}
	})

	t.Run("Write and Read via HandleRequest", func(t *testing.T) {
		// Write File
		writeReq := MCPRequest{
			JSONRPC: "2.0",
			ID:      1,
			Method:  "callTool",
			Params: json.RawMessage(`{
				"name": "write_file",
				"arguments": {
					"path": "mcp_test.txt",
					"content": "mcp content"
				}
			}`),
		}
		reqBytes, _ := json.Marshal(writeReq)
		respBytes, err := server.HandleRequest(ctx, reqBytes)
		if err != nil {
			t.Fatalf("HandleRequest write failed: %v", err)
		}

		var resp MCPResponse
		json.Unmarshal(respBytes, &resp)
		if resp.Error != nil {
			t.Fatalf("HandleRequest write returned error: %v", resp.Error)
		}

		// Read File
		readReq := MCPRequest{
			JSONRPC: "2.0",
			ID:      2,
			Method:  "callTool",
			Params: json.RawMessage(`{
				"name": "read_file",
				"arguments": {
					"path": "mcp_test.txt"
				}
			}`),
		}
		reqBytes, _ = json.Marshal(readReq)
		respBytes, err = server.HandleRequest(ctx, reqBytes)
		if err != nil {
			t.Fatalf("HandleRequest read failed: %v", err)
		}

		json.Unmarshal(respBytes, &resp)
		if resp.Error != nil {
			t.Fatalf("HandleRequest read returned error: %v", resp.Error)
		}

		resultMap, ok := resp.Result.(map[string]interface{})
		if !ok || resultMap["content"] != "mcp content" {
			t.Errorf("expected 'mcp content', got %v", resp.Result)
		}
	})
}
