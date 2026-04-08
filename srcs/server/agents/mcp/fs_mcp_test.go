package mcp

import (
	"context"
	"encoding/json"
	"io/fs"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/stretchr/testify/assert"
)

// mockFSProvider for testing FSMCPServer independently
type mockFSProvider struct {
	readFileData    []byte
	readFileErr     error
	writeFileErr    error
	listDirEntries  []fs.FileInfo
	listDirErr      error
}

func (m *mockFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	return m.readFileData, m.readFileErr
}

func (m *mockFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	return m.writeFileErr
}

func (m *mockFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]fs.FileInfo, error) {
	return m.listDirEntries, m.listDirErr
}

// mockFileInfo implements fs.FileInfo
type mockFileInfo struct {
	name string
}
func (m mockFileInfo) Name() string       { return m.name }
func (m mockFileInfo) Size() int64        { return 0 }
func (m mockFileInfo) Mode() fs.FileMode  { return 0 }
func (m mockFileInfo) ModTime() time.Time { return time.Time{} }
func (m mockFileInfo) IsDir() bool        { return false }
func (m mockFileInfo) Sys() any           { return nil }

func TestFSMCPServer_CallTool_ReadFile(t *testing.T) {
	mock := &mockFSProvider{readFileData: []byte("mock content")}
	server := NewFSMCPServer(mock)

	input := map[string]interface{}{"path": "test.txt"}
	result := server.CallTool(context.Background(), nil, "read_file", input)

	assert.Equal(t, "success", result.Status)

	var data map[string]string
	err := json.Unmarshal(result.ResultData, &data)
	assert.NoError(t, err)
	assert.Equal(t, "mock content", data["content"])
}

func TestFSMCPServer_CallTool_WriteFile(t *testing.T) {
	mock := &mockFSProvider{}
	server := NewFSMCPServer(mock)

	input := map[string]interface{}{"path": "test.txt", "data": "new data"}
	result := server.CallTool(context.Background(), nil, "write_file", input)

	assert.Equal(t, "success", result.Status)

	var data map[string]string
	err := json.Unmarshal(result.ResultData, &data)
	assert.NoError(t, err)
	assert.Equal(t, "success", data["status"])
}

func TestFSMCPServer_CallTool_ListDir(t *testing.T) {
	mock := &mockFSProvider{
		listDirEntries: []fs.FileInfo{
			mockFileInfo{name: "file1.txt"},
			mockFileInfo{name: "file2.go"},
		},
	}
	server := NewFSMCPServer(mock)

	input := map[string]interface{}{"path": "dir"}
	result := server.CallTool(context.Background(), nil, "list_directory", input)

	assert.Equal(t, "success", result.Status)

	var data map[string][]string
	err := json.Unmarshal(result.ResultData, &data)
	assert.NoError(t, err)
	assert.ElementsMatch(t, []string{"file1.txt", "file2.go"}, data["files"])
}

func TestFSMCPServer_CallTool_UnknownTool(t *testing.T) {
	mock := &mockFSProvider{}
	server := NewFSMCPServer(mock)

	input := map[string]interface{}{}
	result := server.CallTool(context.Background(), nil, "unknown_tool", input)

	assert.Equal(t, "error", result.Status)
	assert.Contains(t, string(result.ResultData), "unknown tool")
}
