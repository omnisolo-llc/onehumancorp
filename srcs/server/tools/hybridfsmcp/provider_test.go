package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	dir := t.TempDir()
	p := NewProvider(true, dir)

	ctx := context.Background()
	err := p.WriteFile(ctx, nil, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := p.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got '%s'", string(data))
	}

	entries, err := p.ListDir(ctx, nil, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	_, err = p.ReadFile(ctx, nil, "../outside.txt")
	if err == nil {
		t.Error("expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir := t.TempDir()
	p := NewProvider(false, dir)

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	err := p.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := p.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", string(data))
	}

	entries, err := p.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Ensure isolation
	_, err = os.Stat(filepath.Join(dir, "tenant-1", "test.txt"))
	if err != nil {
		t.Errorf("file not written in tenant directory: %v", err)
	}

	_, err = p.ReadFile(ctx, claims, "../tenant-2/test.txt")
	if err == nil {
		t.Error("expected path traversal error, got nil")
	}

	// Missing claims
	err = p.WriteFile(ctx, nil, "test.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error for missing claims, got nil")
	}
}
