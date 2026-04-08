package hybridfsmcp

import (
	"context"
	"os"
	"testing"
)

func TestLocalFSErrors(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfsmcp_err")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)

	// WriteFile error from resolvePath
	err = provider.WriteFile(context.Background(), "../test.txt", []byte("hi"))
	if err == nil {
		t.Fatal("expected error")
	}

	// ListDir error from resolvePath
	_, err = provider.ListDir(context.Background(), "../test")
	if err == nil {
		t.Fatal("expected error")
	}

	// ListDir error from bad path
	_, err = provider.ListDir(context.Background(), "doesnotexist")
	if err == nil {
		t.Fatal("expected error")
	}

	// SearchFiles error from resolvePath
	_, err = provider.SearchFiles(context.Background(), "../test", "pattern")
	if err == nil {
		t.Fatal("expected error")
	}

	// SearchFiles error from walk
	_, err = provider.SearchFiles(context.Background(), "doesnotexist", "pattern")
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestCloudFSErrors(t *testing.T) {
	provider := NewCloudFSProvider()

	// getTenantPath error (no context claims)
	_, err := provider.ReadFile(context.Background(), "test.txt")
	if err == nil {
		t.Fatal("expected error")
	}

	_, err = provider.ListDir(context.Background(), "test")
	if err == nil {
		t.Fatal("expected error")
	}

	_, err = provider.SearchFiles(context.Background(), "test", "pattern")
	if err == nil {
		t.Fatal("expected error")
	}

	err = provider.WriteFile(context.Background(), "test.txt", []byte("hi"))
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestCallToolErrors(t *testing.T) {
	provider := NewLocalFSProvider("/tmp")
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// read_file provider error
	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "doesnotexist"})
	if err == nil {
		t.Fatal("expected error")
	}

	// write_file provider error
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "../doesnotexist", "content": "hi"})
	if err == nil {
		t.Fatal("expected error")
	}

	// list_directory provider error
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "doesnotexist"})
	if err == nil {
		t.Fatal("expected error")
	}

	// search_files provider error
	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"path": "doesnotexist", "pattern": "test"})
	if err == nil {
		t.Fatal("expected error")
	}
}
