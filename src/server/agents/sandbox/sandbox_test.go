package sandbox

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestSandboxManager_CheckRead(t *testing.T) {
	ctx := context.Background()
	config := SandboxConfig{
		AllowedReadPaths: []string{"/tmp/allowed"},
	}
	sm := NewSandboxManager(config)

	err := sm.CheckRead(ctx, "/tmp/allowed/file.txt")
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	err = sm.CheckRead(ctx, "/etc/passwd")
	if err == nil {
		t.Errorf("Expected error, got nil")
	}

	// Test dangerously disable
	config.DangerouslyDisable = true
	sm = NewSandboxManager(config)
	err = sm.CheckRead(ctx, "/etc/passwd")
	if err != nil {
		t.Errorf("Expected nil when disabled, got %v", err)
	}
}

func TestSandboxManager_CheckWrite(t *testing.T) {
	ctx := context.Background()
	config := SandboxConfig{
		AllowedWritePaths: []string{"/tmp/allowed"},
	}
	sm := NewSandboxManager(config)

	err := sm.CheckWrite(ctx, "/tmp/allowed/file.txt")
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	err = sm.CheckWrite(ctx, "/etc/passwd")
	if err == nil {
		t.Errorf("Expected error, got nil")
	}

	// Test dangerously disable
	config.DangerouslyDisable = true
	sm = NewSandboxManager(config)
	err = sm.CheckWrite(ctx, "/etc/passwd")
	if err != nil {
		t.Errorf("Expected nil when disabled, got %v", err)
	}
}

func TestSandboxManager_CheckNetwork(t *testing.T) {
	ctx := context.Background()
	config := SandboxConfig{
		AllowedHosts: []string{"example.com"},
	}
	sm := NewSandboxManager(config)

	err := sm.CheckNetwork(ctx, "example.com")
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	err = sm.CheckNetwork(ctx, "example.com:80")
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	err = sm.CheckNetwork(ctx, "google.com")
	if err == nil {
		t.Errorf("Expected error, got nil")
	}

	// Test dangerously disable
	config.DangerouslyDisable = true
	sm = NewSandboxManager(config)
	err = sm.CheckNetwork(ctx, "google.com")
	if err != nil {
		t.Errorf("Expected nil when disabled, got %v", err)
	}
}

func TestSandboxManager_ReadFile(t *testing.T) {
	ctx := context.Background()
	tempDir := t.TempDir()
	tempFile := filepath.Join(tempDir, "test.txt")
	content := []byte("hello")
	os.WriteFile(tempFile, content, 0644)

	config := SandboxConfig{
		AllowedReadPaths: []string{tempDir},
	}
	sm := NewSandboxManager(config)

	data, err := sm.ReadFile(ctx, tempFile)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got %s", string(data))
	}

	_, err = sm.ReadFile(ctx, "/etc/passwd")
	if err == nil {
		t.Errorf("Expected error, got nil")
	}
}
