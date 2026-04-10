package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error writing file: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error reading file: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %s", string(data))
	}

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error listing dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Traversal
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hack"))
	if err == nil {
		t.Errorf("expected error for directory traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()

	// No claims
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err == nil {
		t.Errorf("expected error without claims")
	}

	// Valid claims
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-1",
	})

	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error writing file: %v", err)
	}

	data, err := provider.ReadFile(ctxWithClaims, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error reading file: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %s", string(data))
	}

	// Traversal
	err = provider.WriteFile(ctxWithClaims, "../escape.txt", []byte("hack"))
	if err == nil {
		t.Errorf("expected error for directory traversal")
	}

	// Other tenant isolation test
	ctxOtherTenant := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-2",
	})

	_, err = provider.ReadFile(ctxOtherTenant, "test.txt")
	if err == nil {
		t.Errorf("expected error reading other tenant file")
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	p := NewProvider(".")
	if _, ok := p.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider")
	}

	os.Setenv("OHC_MULTITENANT", "false")
	p = NewProvider(".")
	if _, ok := p.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider")
	}
}

func TestServer(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)
	s := NewServer(p)
	ctx := context.Background()

	// write
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Data: []byte("foo")})
	_, err := s.ExecuteTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("unexpected error executing write_file: %v", err)
	}

	// read
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "test.txt"})
	res, err := s.ExecuteTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("unexpected error executing read_file: %v", err)
	}

	var readData map[string][]byte
	json.Unmarshal(res.ResultData, &readData)
	if string(readData["data"]) != "foo" {
		t.Errorf("expected 'foo', got %s", string(readData["data"]))
	}

	// list
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res, err = s.ExecuteTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("unexpected error executing list_directory: %v", err)
	}

	var listData map[string][]string
	json.Unmarshal(res.ResultData, &listData)
	if len(listData["entries"]) != 1 || listData["entries"][0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", listData["entries"])
	}

	// unknown tool
	_, err = s.ExecuteTool(ctx, "unknown", nil)
	if err == nil {
		t.Errorf("expected error for unknown tool")
	}
}
