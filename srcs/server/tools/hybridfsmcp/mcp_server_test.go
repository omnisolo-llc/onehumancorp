package hybridfsmcp

import (
	"context"
	"os"
	"testing"
)

func TestFSMCPServer(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	server := NewFSMCPServer(provider)
	ctx := context.Background()

	// Write
	err = server.HandleWriteFile(ctx, "any", "test.txt", []byte("server"))
	if err != nil {
		t.Errorf("HandleWriteFile failed: %v", err)
	}

	// Read
	data, err := server.HandleReadFile(ctx, "any", "test.txt")
	if err != nil {
		t.Errorf("HandleReadFile failed: %v", err)
	}
	if string(data) != "server" {
		t.Errorf("expected 'server', got %s", data)
	}

	// List
	files, err := server.HandleListDirectory(ctx, "any", ".")
	if err != nil {
		t.Errorf("HandleListDirectory failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("unexpected files: %v", files)
	}

	// Search
	matches, err := server.HandleSearchFiles(ctx, "any", ".", "*.txt")
	if err != nil {
		t.Errorf("HandleSearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("unexpected matches: %v", matches)
	}
}
