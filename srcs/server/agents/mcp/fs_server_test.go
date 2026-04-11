package mcp

import (
    "context"
    "os"
    "path/filepath"
    "testing"
)

func TestFSMCP(t *testing.T) {
    tempDir, err := os.MkdirTemp("", "fsmcp")
    if err != nil {
        t.Fatalf("failed to create temp dir: %v", err)
    }
    defer os.RemoveAll(tempDir)
    absTempDir, _ := filepath.Abs(tempDir)

    provider := NewLocalFSProvider(absTempDir)
    server := NewFSMCP(provider)
    ctx := context.Background()

    // Test WriteFile
    res := server.WriteFile(ctx, "test.txt", []byte("data"), nil)
    if res.Status != "success" {
        t.Errorf("expected success, got %s: %s", res.Status, string(res.ResultData))
    }

    // Test ReadFile
    res = server.ReadFile(ctx, "test.txt", nil)
    if res.Status != "success" || string(res.ResultData) != "data" {
        t.Errorf("expected success 'data', got %s: %s", res.Status, string(res.ResultData))
    }

    // Test ListDir
    res = server.ListDir(ctx, ".", nil)
    if res.Status != "success" {
        t.Errorf("expected success, got %s: %s", res.Status, string(res.ResultData))
    }

    // Test Errors
    res = server.ReadFile(ctx, "notfound.txt", nil)
    if res.Status != "error" {
        t.Errorf("expected error, got %s", res.Status)
    }
}
