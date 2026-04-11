package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_CallTool_ReadFile(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)
	m := NewHybridFSMCP(p)

	p.WriteFile(context.Background(), nil, "file.txt", []byte("hello mcp"))

	args := map[string]interface{}{"path": "file.txt"}
	res, err := m.CallTool(context.Background(), "read_file", args)

	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", res)
	}

	if resMap["status"] != "success" {
		t.Errorf("expected success, got %s", resMap["status"])
	}
	if resMap["data"] != "hello mcp" {
		t.Errorf("expected 'hello mcp', got %s", resMap["data"])
	}
}

func TestHybridFSMCP_CallTool_WriteFile(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)
	m := NewHybridFSMCP(p)

	args := map[string]interface{}{"path": "out.txt", "data": "wrote this"}
	res, err := m.CallTool(context.Background(), "write_file", args)

	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", res)
	}

	if resMap["status"] != "success" {
		t.Errorf("expected success, got %s", resMap["status"])
	}

	data, _ := p.ReadFile(context.Background(), nil, "out.txt")
	if string(data) != "wrote this" {
		t.Errorf("expected 'wrote this', got %s", data)
	}
}

func TestHybridFSMCP_CallTool_ListDir(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)
	m := NewHybridFSMCP(p)

	p.WriteFile(context.Background(), nil, "a.txt", []byte("a"))

	args := map[string]interface{}{"path": ""}
	res, err := m.CallTool(context.Background(), "list_directory", args)

	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", res)
	}

	entries, ok := resMap["entries"].([]string)
	if !ok {
		t.Fatalf("expected []string, got %T", resMap["entries"])
	}

	if len(entries) != 1 || entries[0] != "a.txt" {
		t.Errorf("unexpected entries: %v", entries)
	}
}

func TestHybridFSMCP_CallTool_SearchFiles(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)
	m := NewHybridFSMCP(p)

	p.WriteFile(context.Background(), nil, "match.txt", []byte("a"))
	p.WriteFile(context.Background(), nil, "other.log", []byte("b"))

	args := map[string]interface{}{"query": "match"}
	res, err := m.CallTool(context.Background(), "search_files", args)

	if err != nil {
		t.Fatalf("CallTool search_files failed: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}, got %T", res)
	}

	results, ok := resMap["results"].([]string)
	if !ok {
		t.Fatalf("expected []string, got %T", resMap["results"])
	}

	if len(results) != 1 || results[0] != "match.txt" {
		t.Errorf("unexpected search results: %v", results)
	}
}

func TestHybridFSMCP_CloudMode_RequiresAuth(t *testing.T) {
	p := NewCloudFSProvider()
	m := NewHybridFSMCP(p)

	args := map[string]interface{}{"path": "file.txt"}

	// Should fail with no auth claims
	_, err := m.CallTool(context.Background(), "read_file", args)
	if err == nil {
		t.Fatalf("expected auth error, got nil")
	}

	// With valid claims, should pass auth check (but fail with os.ErrNotExist since it's empty)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-a"})
	_, err = m.CallTool(ctx, "read_file", args)
	if err == nil {
		t.Fatalf("expected not exist error, got nil")
	}
}
