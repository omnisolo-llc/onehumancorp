package hybridfsmcp

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestMCPServer(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewFileSystemProvider("OHC_MULTITENANT", tmpDir)
	server := NewServer(provider)

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Write
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Data: "hello"})
	_, err := server.HandleCall(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Read
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "test.txt"})
	res, err := server.HandleCall(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := res.(map[string]string)
	if !ok || resMap["content"] != "hello" {
		t.Errorf("expected content 'hello', got %v", res)
	}

	// List
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	resList, err := server.HandleCall(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	listStr, ok := resList.([]string)
	if !ok || len(listStr) != 1 {
		t.Errorf("expected 1 item, got %v", resList)
	}
}
