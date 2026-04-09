package mcp

import (
	"context"
	"os"
	"testing"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	p := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	err := p.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := p.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// Path bounds checking
	err = p.WriteFile(ctx, "../escape.txt", []byte("evil"))
	if err == nil {
		t.Error("Expected error when escaping base directory")
	}
}

func TestCloudFSProvider(t *testing.T) {
	p := NewCloudFSProvider("tenant-1")
	ctx := context.Background()

	err := p.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := p.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	p2 := NewCloudFSProvider("tenant-2")
	_, err = p2.ReadFile(ctx, "test.txt")
	if !os.IsNotExist(err) {
		t.Errorf("Expected IsNotExist error across tenants, got %v", err)
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir := t.TempDir()
	p := NewLocalFSProvider(tmpDir)
	mcpServer := NewHybridFSMCP(p)
	ctx := context.Background()

	mcpServer.WriteFile(ctx, "hello.txt", []byte("hybrid"))
	data, _ := mcpServer.ReadFile(ctx, "hello.txt")
	if string(data) != "hybrid" {
		t.Errorf("Expected hybrid, got %s", string(data))
	}
}
