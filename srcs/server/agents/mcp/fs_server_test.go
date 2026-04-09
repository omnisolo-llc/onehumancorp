package mcp

import (
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestFileSystemServer_ExecuteTool(t *testing.T) {
	tempDir := t.TempDir()
	claims := &auth.Claims{OrganizationID: "org-123"}

	s := NewFileSystemServer(tempDir, claims, true)

	// Write
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Content: "hello server"})
	res := s.ExecuteTool("write_file", writeArgs)
	if res.Status != "success" {
		t.Errorf("Expected success, got %s: %s", res.Status, res.ResultData)
	}

	// Read
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "test.txt"})
	res = s.ExecuteTool("read_file", readArgs)
	if res.Status != "success" {
		t.Errorf("Expected success, got %s", res.Status)
	}

	var readOut map[string]string
	json.Unmarshal(res.ResultData, &readOut)
	if readOut["content"] != "hello server" {
		t.Errorf("Expected 'hello server', got '%s'", readOut["content"])
	}

	// List
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res = s.ExecuteTool("list_directory", listArgs)
	if res.Status != "success" {
		t.Errorf("Expected success, got %s", res.Status)
	}

	var listOut map[string][]string
	json.Unmarshal(res.ResultData, &listOut)
	if len(listOut["entries"]) != 1 || listOut["entries"][0] != "test.txt" {
		t.Errorf("Expected [test.txt], got %v", listOut["entries"])
	}

	// Error case
	res = s.ExecuteTool("unknown", []byte("{}"))
	if res.Status != "error" {
		t.Errorf("Expected error status, got %s", res.Status)
	}
}
