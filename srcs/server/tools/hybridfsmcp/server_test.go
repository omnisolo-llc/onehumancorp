package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

func TestServer(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcpserver")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := mcp.NewFileSystemProvider(tmpDir) // LocalFSProvider by default
	server := NewServer(provider)

	ctx := context.Background()

	err = server.WriteFile(ctx, "test.txt", []byte("data"))
	if err != nil {
		t.Fatal(err)
	}

	data, err := server.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "data" {
		t.Fatalf("expected 'data', got %s", string(data))
	}

	files, err := server.ListDirectory(ctx, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Fatalf("expected ['test.txt'], got %v", files)
	}
}
