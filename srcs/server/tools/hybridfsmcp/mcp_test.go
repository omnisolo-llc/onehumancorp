package hybridfsmcp

import (
	"testing"
)

func TestHybridFSMCP_ListTools(t *testing.T) {
	dir := t.TempDir()
	provider := NewLocalFSProvider(dir)
	mcp := NewHybridFSMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Fatalf("Expected 4 tools, got %d", len(tools))
	}

	toolNames := map[string]bool{}
	for _, tool := range tools {
		toolNames[tool.Name] = true
	}

	expected := []string{"read_file", "write_file", "list_directory", "search_files"}
	for _, name := range expected {
		if !toolNames[name] {
			t.Fatalf("Missing tool: %s", name)
		}
	}
}
