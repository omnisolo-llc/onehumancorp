package hybridfsmcp

import (
	"context"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmp := t.TempDir()
	p := NewLocalFSProvider(tmp)

	ctx := context.Background()

	err := p.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatal(err)
	}

	data, err := p.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected hello, got %s", string(data))
	}

	entries, err := p.ListDir(ctx, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("expected [test.txt], got %v", entries)
	}

	// Traversal
	err = p.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Fatal("expected traversal error")
	}

	// Sibling
	tmpSibling := tmp + "_secrets"
	err = p.WriteFile(ctx, filepath.Join("..", filepath.Base(tmpSibling), "secret.txt"), []byte("bad"))
	if err == nil {
		t.Fatal("expected traversal error")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmp := t.TempDir()
	p := NewCloudFSProvider(tmp)

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := p.WriteFile(ctx, "test.txt", []byte("hello tenant"))
	if err != nil {
		t.Fatal(err)
	}

	data, err := p.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "hello tenant" {
		t.Fatalf("expected hello tenant, got %s", string(data))
	}

	// Missing auth
	ctxNoAuth := context.Background()
	_, err = p.ReadFile(ctxNoAuth, "test.txt")
	if err == nil {
		t.Fatal("expected unauthorized error")
	}

	// Another tenant
	claims2 := &auth.Claims{
		OrganizationID: "tenant2",
	}
	ctx2 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims2)
	_, err = p.ReadFile(ctx2, "../tenant1/test.txt")
	if err == nil {
		t.Fatal("expected path access denied")
	}
}

func TestNewProvider(t *testing.T) {
	tmp := t.TempDir()

	t.Setenv("OHC_STANDALONE", "true")
	p1 := NewProvider(tmp)
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Fatal("expected LocalFSProvider")
	}

	t.Setenv("OHC_STANDALONE", "false")
	p2 := NewProvider(tmp)
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Fatal("expected CloudFSProvider")
	}
}

func TestServer(t *testing.T) {
	tmp := t.TempDir()
	p := NewLocalFSProvider(tmp)
	s := NewServer(p)

	ctx := context.Background()
	_, err := s.HandleCall(ctx, "write_file", []byte(`{"path":"test.txt","content":"data"}`))
	if err != nil {
		t.Fatal(err)
	}

	res, err := s.HandleCall(ctx, "read_file", []byte(`{"path":"test.txt"}`))
	if err != nil {
		t.Fatal(err)
	}
	if res.(string) != "data" {
		t.Fatalf("expected data, got %v", res)
	}

	res, err = s.HandleCall(ctx, "list_directory", []byte(`{"path":"."}`))
	if err != nil {
		t.Fatal(err)
	}
	entries := res.([]string)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("expected [test.txt], got %v", entries)
	}

	_, err = s.HandleCall(ctx, "unknown_tool", []byte(`{}`))
	if err == nil {
		t.Fatal("expected error")
	}
}
