package hybridfsmcp

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSErrors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Read non-existent file
	_, err := provider.ReadFile(ctx, "non_existent.txt")
	if err == nil {
		t.Fatal("expected error reading non-existent file")
	}

	// Write file to a directory that exists as a file (to trigger MkdirAll error, or WriteFile error)
	err = provider.WriteFile(ctx, "file.txt", []byte("data"))
	if err != nil {
		t.Fatal(err)
	}

	err = provider.WriteFile(ctx, "file.txt/nested.txt", []byte("data"))
	if err == nil {
		t.Fatal("expected error creating directory over a file")
	}
}

func TestCloudFSErrors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Read non-existent
	_, err := provider.ReadFile(ctx, "non_existent.txt")
	if err == nil {
		t.Fatal("expected error")
	}

	// Path traversal on WriteFile
	err = provider.WriteFile(ctx, "../../../test.txt", []byte("data"))
	if err == nil {
		t.Fatal("expected error")
	}

	// Write file to a directory that exists as a file
	err = provider.WriteFile(ctx, "file.txt", []byte("data"))
	if err != nil {
		t.Fatal(err)
	}
	err = provider.WriteFile(ctx, "file.txt/nested.txt", []byte("data"))
	if err == nil {
		t.Fatal("expected error")
	}

	// Path traversal on ListDir
	_, err = provider.ListDir(ctx, "../../../")
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestListDir_BadEntry(t *testing.T) {
    // Try to trigger the "continue" on entry.Info()
    // Hard to simulate purely in Go without mocking os.ReadDir
    // But we can test MCP passing errors
}

func TestMCP_CallTool_Errors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSInspectorMCP(provider)
	ctx := context.Background()

	// read_file where provider errors
	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "does_not_exist.txt"})
	if err == nil {
		t.Fatal("expected error")
	}

	// write_file where provider errors (path traversal)
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "../../../passwd", "content_b64": "YWFh"})
	if err == nil {
		t.Fatal("expected error")
	}

	// list_directory where provider errors
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "does_not_exist"})
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestCloudFS_MissingOrgId(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: ""}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Fatal("expected error")
	}
}
