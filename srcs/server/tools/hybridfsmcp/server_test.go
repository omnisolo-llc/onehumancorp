package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestHybridFSServer(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "fs_server")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &LocalFSProvider{BaseDir: tmpDir}
	server := NewHybridFSServer(provider)
	ctx := context.Background()

	// Test write_file
	writeArgs := []byte(`{"path": "test.txt", "data": "content"}`)
	res := server.Call(ctx, "write_file", writeArgs)
	if res.Status != "success" {
		t.Fatalf("expected success, got %s: %s", res.Status, res.ResultData)
	}

	// Test read_file
	readArgs := []byte(`{"path": "test.txt"}`)
	res = server.Call(ctx, "read_file", readArgs)
	if res.Status != "success" {
		t.Fatalf("expected success, got %s", res.Status)
	}
	var readOut map[string]string
	json.Unmarshal(res.ResultData, &readOut)
	if readOut["content"] != "content" {
		t.Errorf("expected 'content', got %q", readOut["content"])
	}

	// Test list_directory
	listArgs := []byte(`{"path": "."}`)
	res = server.Call(ctx, "list_directory", listArgs)
	if res.Status != "success" {
		t.Fatalf("expected success, got %s", res.Status)
	}
	var listOut map[string][]string
	json.Unmarshal(res.ResultData, &listOut)
	if len(listOut["files"]) != 1 || listOut["files"][0] != "test.txt" {
		t.Errorf("unexpected files: %v", listOut["files"])
	}

	// Test unknown tool
	res = server.Call(ctx, "unknown", []byte(`{}`))
	if res.Status != "error" {
		t.Errorf("expected error for unknown tool")
	}

	// Test bad json args
	res = server.Call(ctx, "read_file", []byte(`{bad json`))
	if res.Status != "error" {
		t.Errorf("expected error for bad json")
	}
}

func TestHybridFSServer_ListDirErrors(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "fs_server")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &LocalFSProvider{BaseDir: tmpDir}
	server := NewHybridFSServer(provider)
	ctx := context.Background()

	// ListDir bad path
	res := server.Call(ctx, "list_directory", []byte(`{"path": "../escape"}`))
	if res.Status != "error" {
		t.Errorf("expected error for bad path in list_directory")
	}

	// ListDir bad json
	res = server.Call(ctx, "list_directory", []byte(`{bad`))
	if res.Status != "error" {
		t.Errorf("expected error for bad json in list_directory")
	}
}

func TestHybridFSServer_WriteFileErrors(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "fs_server")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &LocalFSProvider{BaseDir: tmpDir}
	server := NewHybridFSServer(provider)
	ctx := context.Background()

	// WriteFile bad path
	res := server.Call(ctx, "write_file", []byte(`{"path": "../escape.txt", "data": "c"}`))
	if res.Status != "error" {
		t.Errorf("expected error for bad path in write_file")
	}

	// WriteFile bad json
	res = server.Call(ctx, "write_file", []byte(`{bad`))
	if res.Status != "error" {
		t.Errorf("expected error for bad json in write_file")
	}
}

func TestHybridFSServer_ReadFileErrors(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "fs_server")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &LocalFSProvider{BaseDir: tmpDir}
	server := NewHybridFSServer(provider)
	ctx := context.Background()

	// ReadFile bad path
	res := server.Call(ctx, "read_file", []byte(`{"path": "../escape.txt"}`))
	if res.Status != "error" {
		t.Errorf("expected error for bad path in read_file")
	}
}
