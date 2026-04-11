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
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := &LocalFSProvider{base: tempDir}
	ctx := context.Background()

	t.Run("Write and Read File", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
		if err != nil {
			t.Fatalf("WriteFile failed: %v", err)
		}

		content, err := provider.ReadFile(ctx, nil, "test.txt")
		if err != nil {
			t.Fatalf("ReadFile failed: %v", err)
		}
		if string(content) != "hello" {
			t.Errorf("Expected 'hello', got '%s'", string(content))
		}
	})

	t.Run("List Directory", func(t *testing.T) {
		provider.WriteFile(ctx, nil, "dir/file1.txt", []byte("1"))
		provider.WriteFile(ctx, nil, "dir/file2.txt", []byte("2"))

		entries, err := provider.ListDir(ctx, nil, "dir")
		if err != nil {
			t.Fatalf("ListDir failed: %v", err)
		}
		if len(entries) != 2 {
			t.Errorf("Expected 2 entries, got %d", len(entries))
		}
	})

	t.Run("Path Traversal Blocked", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, nil, "../outside.txt")
		if err == nil {
			t.Error("Expected error for path traversal, got nil")
		}

		_, err = provider.ReadFile(ctx, nil, "/etc/passwd")
		if err == nil {
			t.Error("Expected error for absolute path, got nil")
		}
	})
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := &CloudFSProvider{base: tempDir}
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}
	claimsOther := &auth.Claims{OrganizationID: "tenant2"}

	t.Run("Write and Read File Tenant Isolation", func(t *testing.T) {
		err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello tenant1"))
		if err != nil {
			t.Fatalf("WriteFile failed: %v", err)
		}

		// Read as tenant1
		content, err := provider.ReadFile(ctx, claims, "test.txt")
		if err != nil {
			t.Fatalf("ReadFile failed: %v", err)
		}
		if string(content) != "hello tenant1" {
			t.Errorf("Expected 'hello tenant1', got '%s'", string(content))
		}

		// Read as tenant2
		_, err = provider.ReadFile(ctx, claimsOther, "test.txt")
		if err == nil {
			t.Error("Expected error when reading other tenant's file, got nil")
		}
	})

	t.Run("Path Traversal Blocked", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, claims, "../tenant2/test.txt")
		if err == nil {
			t.Error("Expected error for path traversal, got nil")
		}

		_, err = provider.ReadFile(ctx, claims, "/etc/passwd")
		if err == nil {
			t.Error("Expected error for absolute path, got nil")
		}
	})

	t.Run("Missing Claims", func(t *testing.T) {
		_, err := provider.ReadFile(ctx, nil, "test.txt")
		if err == nil {
			t.Error("Expected error for missing claims, got nil")
		}
	})
}

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcpfs")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := &LocalFSProvider{base: tempDir}
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	t.Run("ListTools", func(t *testing.T) {
		tools := mcp.ListTools()
		if len(tools) != 3 {
			t.Errorf("Expected 3 tools, got %d", len(tools))
		}
	})

	t.Run("CallTool Write and Read", func(t *testing.T) {
		res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
			"path":    "test.txt",
			"content": "mcp hello",
		})
		if err != nil {
			t.Fatalf("CallTool write_file failed: %v", err)
		}
		if res.(map[string]interface{})["status"] != "success" {
			t.Error("Expected success status")
		}

		resRead, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
			"path": "test.txt",
		})
		if err != nil {
			t.Fatalf("CallTool read_file failed: %v", err)
		}
		if resRead.(map[string]interface{})["content"] != "mcp hello" {
			t.Errorf("Expected 'mcp hello', got %v", resRead.(map[string]interface{})["content"])
		}
	})

	t.Run("CallTool ListDir", func(t *testing.T) {
		mcp.CallTool(ctx, "write_file", map[string]interface{}{
			"path":    "dir/test.txt",
			"content": "mcp hello",
		})
		res, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
			"path": "dir",
		})
		if err != nil {
			t.Fatalf("CallTool list_directory failed: %v", err)
		}
		entries := res.(map[string]interface{})["entries"].([]string)
		if len(entries) != 1 || entries[0] != "test.txt" {
			t.Errorf("Expected [test.txt], got %v", entries)
		}
	})
}

func TestNewFileSystemProvider(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	p1 := NewFileSystemProvider()
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Error("Expected LocalFSProvider")
	}

	os.Setenv("OHC_MULTITENANT", "true")
	p2 := NewFileSystemProvider()
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Error("Expected CloudFSProvider")
	}
}