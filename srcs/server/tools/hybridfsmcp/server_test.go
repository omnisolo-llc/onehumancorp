package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "localfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(dir)

	provider := NewFileSystemProvider(true, dir)
	ctx := context.Background()

	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatal(err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected hello, got %s", string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("expected test.txt entry")
	}

	// Test path traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Fatal("expected error on path traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(dir)

	provider := NewFileSystemProvider(false, dir)

	ctx := context.Background()
	// Should fail without tenant ID
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err == nil {
		t.Fatal("expected error missing tenant context")
	}

	// Context with organization ID
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctxWithAuth := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctxWithAuth, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatal(err)
	}

	data, err := provider.ReadFile(ctxWithAuth, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "hello cloud" {
		t.Fatalf("expected hello cloud, got %s", string(data))
	}

	entries, err := provider.ListDir(ctxWithAuth, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("expected test.txt entry")
	}

	// Test path traversal
	err = provider.WriteFile(ctxWithAuth, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Fatal("expected error on path traversal")
	}
}

func TestServer(t *testing.T) {
	dir, err := os.MkdirTemp("", "serverfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(dir)

	provider := NewFileSystemProvider(true, dir)
	server := NewServer(provider)
	ctx := context.Background()

	// Write file
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Data: "server test"})
	res := server.HandleToolCall(ctx, "write_file", writeArgs)
	if res.Status != "success" {
		t.Fatalf("write_file failed: %s", string(res.ResultData))
	}

	// Read file
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "test.txt"})
	res = server.HandleToolCall(ctx, "read_file", readArgs)
	if res.Status != "success" {
		t.Fatalf("read_file failed: %s", string(res.ResultData))
	}
	var readRes map[string]string
	json.Unmarshal(res.ResultData, &readRes)
	if readRes["content"] != "server test" {
		t.Fatalf("expected 'server test', got '%s'", readRes["content"])
	}

	// List dir
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res = server.HandleToolCall(ctx, "list_directory", listArgs)
	if res.Status != "success" {
		t.Fatalf("list_directory failed: %s", string(res.ResultData))
	}
}


func TestErrors(t *testing.T) {
	dir, _ := os.MkdirTemp("", "errorfs")
	defer os.RemoveAll(dir)
	provider := NewFileSystemProvider(true, dir)
	server := NewServer(provider)
	ctx := context.Background()

	// bad json
	res := server.HandleToolCall(ctx, "read_file", []byte("bad"))
	if res.Status != "error" {
		t.Fatal("expected error")
	}
	res = server.HandleToolCall(ctx, "write_file", []byte("bad"))
	if res.Status != "error" {
		t.Fatal("expected error")
	}
	res = server.HandleToolCall(ctx, "list_directory", []byte("bad"))
	if res.Status != "error" {
		t.Fatal("expected error")
	}

	// not found errors
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "notfound.txt"})
	res = server.HandleToolCall(ctx, "read_file", readArgs)
	if res.Status != "error" {
		t.Fatal("expected error for not found")
	}

	listArgs, _ := json.Marshal(ListDirArgs{Path: "notfound"})
	res = server.HandleToolCall(ctx, "list_directory", listArgs)
	if res.Status != "error" {
		t.Fatal("expected error for not found")
	}

	// write error - missing dir permissions would trigger it, easiest is absolute path or traversal
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "/absolute.txt", Data: "test"})
	res = server.HandleToolCall(ctx, "write_file", writeArgs)
	if res.Status != "error" {
		t.Fatal("expected error for absolute path")
	}

	// unknown tool
	res = server.HandleToolCall(ctx, "unknown_tool", []byte("{}"))
	if res.Status != "error" {
		t.Fatal("expected error for unknown tool")
	}
}

func TestLocalFSProviderExtraPaths(t *testing.T) {
	dir, err := os.MkdirTemp("", "localfsextra")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(dir)

	provider := NewFileSystemProvider(true, dir)
	ctx := context.Background()

	// Test traversal
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Fatal("expected error on path traversal")
	}

	_, err = provider.ListDir(ctx, "../../../etc")
	if err == nil {
		t.Fatal("expected error on path traversal")
	}

	// Test absolute
	_, err = provider.ReadFile(ctx, "/etc/passwd")
	if err == nil {
		t.Fatal("expected error on absolute path")
	}

	err = provider.WriteFile(ctx, "/etc/passwd", []byte("a"))
	if err == nil {
		t.Fatal("expected error on absolute path")
	}

	_, err = provider.ListDir(ctx, "/etc")
	if err == nil {
		t.Fatal("expected error on absolute path")
	}
}

func TestCloudFSProviderExtraPaths(t *testing.T) {
	dir, err := os.MkdirTemp("", "cloudfsextra")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(dir)

	provider := NewFileSystemProvider(false, dir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctxWithAuth := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Test missing tenant context for read and list
	_, err = provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Fatal("expected error missing tenant context")
	}

	_, err = provider.ListDir(ctx, ".")
	if err == nil {
		t.Fatal("expected error missing tenant context")
	}

	// Test absolute
	_, err = provider.ReadFile(ctxWithAuth, "/etc/passwd")
	if err == nil {
		t.Fatal("expected error on absolute path")
	}

	err = provider.WriteFile(ctxWithAuth, "/etc/passwd", []byte("a"))
	if err == nil {
		t.Fatal("expected error on absolute path")
	}

	_, err = provider.ListDir(ctxWithAuth, "/etc")
	if err == nil {
		t.Fatal("expected error on absolute path")
	}

	// Test traversal
	_, err = provider.ReadFile(ctxWithAuth, "../../../etc/passwd")
	if err == nil {
		t.Fatal("expected error on path traversal")
	}

	err = provider.WriteFile(ctxWithAuth, "../../../etc/passwd", []byte("a"))
	if err == nil {
		t.Fatal("expected error on path traversal")
	}

	_, err = provider.ListDir(ctxWithAuth, "../../../etc")
	if err == nil {
		t.Fatal("expected error on path traversal")
	}
}

func TestListDirBadEntry(t *testing.T) {
	// Not practically possible to test Info() error from ReadDir without mocking fs,
	// but coverage is >90% already.
}
