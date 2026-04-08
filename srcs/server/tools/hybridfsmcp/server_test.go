package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

func TestServerFactory(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "factory_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_STANDALONE", "true")
	server := NewHybridFSMCP(tempDir)
	if _, ok := server.provider.(*mcp.LocalFSProvider); !ok {
		t.Fatalf("expected LocalFSProvider when OHC_STANDALONE is true")
	}

	os.Setenv("OHC_STANDALONE", "false")
	server = NewHybridFSMCP(tempDir)
	if _, ok := server.provider.(*mcp.CloudFSProvider); !ok {
		t.Fatalf("expected CloudFSProvider when OHC_STANDALONE is false")
	}
	os.Unsetenv("OHC_STANDALONE")
}

func TestServerTools(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "servertools_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_STANDALONE", "true")
	server := NewHybridFSMCP(tempDir)

	ctx := context.Background()

	// WriteFile
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Data: "hello world"})
	_, err = server.WriteFile(ctx, writeArgs)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// ReadFile
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "test.txt"})
	res, err := server.ReadFile(ctx, readArgs)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	var readRes map[string]string
	json.Unmarshal(res.ResultData, &readRes)
	if readRes["content"] != "hello world" {
		t.Fatalf("expected 'hello world', got '%s'", readRes["content"])
	}

	// ListDir
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res, err = server.ListDir(ctx, listArgs)
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	var listRes map[string]interface{}
	json.Unmarshal(res.ResultData, &listRes)
	entries := listRes["entries"].([]interface{})
	if len(entries) != 1 || entries[0].(string) != "test.txt" {
		t.Fatalf("unexpected list dir result: %v", entries)
	}

	// SearchFiles
	searchArgs, _ := json.Marshal(SearchFilesArgs{Path: ".", Pattern: "test"})
	res, err = server.SearchFiles(ctx, searchArgs)
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	var searchRes map[string]interface{}
	json.Unmarshal(res.ResultData, &searchRes)
	matches := searchRes["matches"].([]interface{})
	if len(matches) != 1 || filepath.Base(matches[0].(string)) != "test.txt" {
		t.Fatalf("unexpected search files result: %v", matches)
	}
}
