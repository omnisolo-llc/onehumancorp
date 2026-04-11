package hybridfsmcp



import (
	"context"
	"os"
	"encoding/json"
	"testing"
)

type MockProvider struct {
	written map[string][]byte
}

func NewMockProvider() *MockProvider {
	return &MockProvider{written: make(map[string][]byte)}
}

func (m *MockProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	if data, ok := m.written[path]; ok {
		return data, nil
	}
	return nil, os.ErrNotExist
}

func (m *MockProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	m.written[path] = data
	return nil
}

func (m *MockProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	var keys []string
	for k := range m.written {
		keys = append(keys, k)
	}
	return keys, nil
}

func TestFileSystemMCPServer(t *testing.T) {
	mock := NewMockProvider()
	server := NewFileSystemMCPServer(mock)
	ctx := context.Background()

	// Test write
	writeArgs := []byte(`{"path": "test.txt", "content": "hello mcp"}`)
	res, err := server.HandleWriteFile(ctx, writeArgs)
	if err != nil {
		t.Fatalf("HandleWriteFile failed: %v", err)
	}
	if res.ToolID != "write_file" || res.Status != "success" {
		t.Errorf("Unexpected result: %+v", res)
	}

	// Test read
	readArgs := []byte(`{"path": "test.txt"}`)
	res, err = server.HandleReadFile(ctx, readArgs)
	if err != nil {
		t.Fatalf("HandleReadFile failed: %v", err)
	}
	if res.ToolID != "read_file" || res.Status != "success" {
		t.Errorf("Unexpected result: %+v", res)
	}

	var readData map[string]string
	json.Unmarshal(res.ResultData, &readData)
	if readData["content"] != "hello mcp" {
		t.Errorf("Expected 'hello mcp', got '%s'", readData["content"])
	}

	// Test ListDir
	listArgs := []byte(`{"path": "."}`)
	res, err = server.HandleListDirectory(ctx, listArgs)
	if err != nil {
		t.Fatalf("HandleListDirectory failed: %v", err)
	}
	if res.ToolID != "list_directory" || res.Status != "success" {
		t.Errorf("Unexpected result: %+v", res)
	}
}
