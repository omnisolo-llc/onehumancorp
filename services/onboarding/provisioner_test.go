package onboarding

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestProvisionEnvironment_Local(t *testing.T) {
	err := ProvisionEnvironment(context.Background(), false)
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}

	expectedDirs := []string{
		filepath.Join(".ohc-local-data", "db"),
		filepath.Join(".ohc-local-data", "blob"),
		filepath.Join(".ohc-local-data", "config"),
	}

	for _, dir := range expectedDirs {
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			t.Errorf("expected directory %s to exist", dir)
		}
	}

	os.RemoveAll(".ohc-local-data")
}

func TestProvisionEnvironment_Cloud(t *testing.T) {
	err := ProvisionEnvironment(context.Background(), true)
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}

	expectedDirs := []string{
		filepath.Join(".ohc-cloud-data", "db"),
		filepath.Join(".ohc-cloud-data", "blob"),
		filepath.Join(".ohc-cloud-data", "config"),
	}

	for _, dir := range expectedDirs {
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			t.Errorf("expected directory %s to exist", dir)
		}
	}

	os.RemoveAll(".ohc-cloud-data")
}
