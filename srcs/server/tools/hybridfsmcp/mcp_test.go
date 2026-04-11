package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"errors"
	"reflect"
	"testing"
)

type MockProvider struct {
	readFileCalled  bool
	writeFileCalled bool
	listDirCalled   bool
	pathArg         string
	dataArg         []byte
}

func (m *MockProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	m.readFileCalled = true
	m.pathArg = path
	if path == "error.txt" {
		return nil, errors.New("read error")
	}
	return []byte("mock data"), nil
}

func (m *MockProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	m.writeFileCalled = true
	m.pathArg = path
	m.dataArg = data
	if path == "error.txt" {
		return errors.New("write error")
	}
	return nil
}

func (m *MockProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	m.listDirCalled = true
	m.pathArg = path
	if path == "error_dir" {
		return nil, errors.New("listdir error")
	}
	return []string{"file1.txt", "file2.txt"}, nil
}

func TestHybridFSMCP_ListTools(t *testing.T) {
	provider := &MockProvider{}
	mcp := NewHybridFSMCP(provider)
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}

	toolNames := []string{}
	for _, tool := range tools {
		toolNames = append(toolNames, tool.Name)
	}

	expectedNames := []string{"read_file", "write_file", "list_directory"}
	if !reflect.DeepEqual(toolNames, expectedNames) {
		t.Errorf("Expected tool names %v, got %v", expectedNames, toolNames)
	}

	// Verify schema presence
	for _, tool := range tools {
		if len(tool.InputSchema) == 0 {
			t.Errorf("Expected InputSchema for tool %s", tool.Name)
		}
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	provider := &MockProvider{}
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// read_file
	t.Run("read_file", func(t *testing.T) {
		res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
			"path": "test.txt",
		})
		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}

		if !provider.readFileCalled {
			t.Error("Expected provider.ReadFile to be called")
		}

		resMap, ok := res.(map[string]interface{})
		if !ok {
			t.Fatalf("Expected map[string]interface{}, got %T", res)
		}

		if resMap["status"] != "success" {
			t.Errorf("Expected status 'success', got %v", resMap["status"])
		}

		encodedData := base64.StdEncoding.EncodeToString([]byte("mock data"))
		if resMap["data"] != encodedData {
			t.Errorf("Expected data '%s', got %v", encodedData, resMap["data"])
		}
	})

	// write_file
	t.Run("write_file", func(t *testing.T) {
		provider.writeFileCalled = false // reset
		encodedData := base64.StdEncoding.EncodeToString([]byte("new data"))

		res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
			"path": "out.txt",
			"data": encodedData,
		})

		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}

		if !provider.writeFileCalled {
			t.Error("Expected provider.WriteFile to be called")
		}

		if string(provider.dataArg) != "new data" {
			t.Errorf("Expected written data to be 'new data', got '%s'", string(provider.dataArg))
		}

		resMap, ok := res.(map[string]interface{})
		if !ok {
			t.Fatalf("Expected map[string]interface{}, got %T", res)
		}

		if resMap["status"] != "success" {
			t.Errorf("Expected status 'success', got %v", resMap["status"])
		}
	})

	// list_directory
	t.Run("list_directory", func(t *testing.T) {
		provider.listDirCalled = false // reset
		res, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
			"path": "dir",
		})

		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}

		if !provider.listDirCalled {
			t.Error("Expected provider.ListDir to be called")
		}

		resMap, ok := res.(map[string]interface{})
		if !ok {
			t.Fatalf("Expected map[string]interface{}, got %T", res)
		}

		if resMap["status"] != "success" {
			t.Errorf("Expected status 'success', got %v", resMap["status"])
		}

		entries, ok := resMap["entries"].([]string)
		if !ok {
			t.Fatalf("Expected []string, got %T", resMap["entries"])
		}

		if !reflect.DeepEqual(entries, []string{"file1.txt", "file2.txt"}) {
			t.Errorf("Expected ['file1.txt', 'file2.txt'], got %v", entries)
		}
	})

	// unknown tool
	t.Run("unknown_tool", func(t *testing.T) {
		_, err := mcp.CallTool(ctx, "invalid", map[string]interface{}{})
		if err == nil {
			t.Error("Expected error for unknown tool, got nil")
		}
	})
}
