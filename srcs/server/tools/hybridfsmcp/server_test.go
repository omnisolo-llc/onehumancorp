package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

func TestHybridFSServer(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "hybridfsserver")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	t.Run("Local Mode", func(t *testing.T) {
		os.Setenv("OHC_MULTITENANT", "false")
		server, err := NewHybridFSServer()
		if err != nil {
			t.Fatalf("failed to create server: %v", err)
		}

		ctx := context.Background()

		// Write
		writeArgs := []byte(`{"path": "foo/bar.txt", "content": "testdata"}`)
		res := server.HandleToolCall(ctx, "write_file", writeArgs)
		if res.Status != "success" {
			t.Fatalf("write_file failed: %s", string(res.ResultData))
		}

		// Read
		readArgs := []byte(`{"path": "foo/bar.txt"}`)
		res = server.HandleToolCall(ctx, "read_file", readArgs)
		if res.Status != "success" {
			t.Fatalf("read_file failed: %s", string(res.ResultData))
		}
		var readRes map[string]string
		if err := json.Unmarshal(res.ResultData, &readRes); err != nil {
			t.Fatalf("failed to parse result: %v", err)
		}
		if readRes["content"] != "testdata" {
			t.Errorf("expected 'testdata', got %s", readRes["content"])
		}

		// List
		listArgs := []byte(`{"path": "foo"}`)
		res = server.HandleToolCall(ctx, "list_directory", listArgs)
		if res.Status != "success" {
			t.Fatalf("list_directory failed: %s", string(res.ResultData))
		}
		var listRes map[string]interface{}
		if err := json.Unmarshal(res.ResultData, &listRes); err != nil {
			t.Fatalf("failed to parse result: %v", err)
		}
		files := listRes["files"].([]interface{})
		if len(files) != 1 {
			t.Errorf("expected 1 file, got %d", len(files))
		}

		// Search
		searchArgs := []byte(`{"path": "", "pattern": "bar"}`)
		res = server.HandleToolCall(ctx, "search_files", searchArgs)
		if res.Status != "success" {
			t.Fatalf("search_files failed: %s", string(res.ResultData))
		}
		var searchRes map[string]interface{}
		if err := json.Unmarshal(res.ResultData, &searchRes); err != nil {
			t.Fatalf("failed to parse result: %v", err)
		}
		matches := searchRes["matches"].([]interface{})
		if len(matches) != 1 {
			t.Errorf("expected 1 match, got %d", len(matches))
		}
		matchPath := matches[0].(string)
		expectedMatchPath := filepath.Join("foo", "bar.txt")
		if matchPath != expectedMatchPath {
			t.Errorf("expected match path %s, got %s", expectedMatchPath, matchPath)
		}
	})
}
