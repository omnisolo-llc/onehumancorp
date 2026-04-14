package onboarding

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestProvisionEnvironment_Local(t *testing.T) {
	LocalBaseDir = t.TempDir()

	err := ProvisionEnvironment(context.Background(), false)
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}

	expectedDirs := []string{
		filepath.Join(LocalBaseDir, "db"),
		filepath.Join(LocalBaseDir, "blob"),
		filepath.Join(LocalBaseDir, "config"),
	}

	for _, dir := range expectedDirs {
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			t.Errorf("expected directory %s to exist", dir)
		}
	}
}

func TestProvisionEnvironment_Cloud(t *testing.T) {
	CloudBaseDir = t.TempDir()

	err := ProvisionEnvironment(context.Background(), true)
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}

	expectedDirs := []string{
		filepath.Join(CloudBaseDir, "db"),
		filepath.Join(CloudBaseDir, "blob"),
		filepath.Join(CloudBaseDir, "config"),
	}

	for _, dir := range expectedDirs {
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			t.Errorf("expected directory %s to exist", dir)
		}
	}
}

func TestCheckEnvironment_Local(t *testing.T) {
	LocalBaseDir = t.TempDir()

	err := CheckEnvironment(false)
	if err == nil {
		t.Fatalf("expected error for missing environment, got nil")
	}

	ProvisionEnvironment(context.Background(), false)
	err = CheckEnvironment(false)
	if err != nil {
		t.Fatalf("expected nil error for provisioned environment, got %v", err)
	}
}

func TestCheckEnvironment_Cloud(t *testing.T) {
	CloudBaseDir = t.TempDir()

	err := CheckEnvironment(true)
	if err == nil {
		t.Fatalf("expected error for missing environment, got nil")
	}

	ProvisionEnvironment(context.Background(), true)
	err = CheckEnvironment(true)
	if err != nil {
		t.Fatalf("expected nil error for provisioned environment, got %v", err)
	}
}

func TestCleanupEnvironment_Local(t *testing.T) {
	LocalBaseDir = t.TempDir()
	ProvisionEnvironment(context.Background(), false)
	err := CleanupEnvironment(context.Background(), false)
	if err != nil {
		t.Fatalf("expected nil error for cleanup environment, got %v", err)
	}
	if err := CheckEnvironment(false); err == nil {
		t.Fatalf("expected error for missing environment after cleanup, got nil")
	}
}

func TestCleanupEnvironment_Cloud(t *testing.T) {
	CloudBaseDir = t.TempDir()
	ProvisionEnvironment(context.Background(), true)
	err := CleanupEnvironment(context.Background(), true)
	if err != nil {
		t.Fatalf("expected nil error for cleanup environment, got %v", err)
	}
	if err := CheckEnvironment(true); err == nil {
		t.Fatalf("expected error for missing environment after cleanup, got nil")
	}
}
