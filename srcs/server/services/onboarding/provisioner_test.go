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
		filepath.Join(".ohc-local-data", "telemetry"),
	}

	for _, dir := range expectedDirs {
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			t.Errorf("expected directory %s to exist", dir)
		}
	}

	if _, err := os.Stat(filepath.Join(".ohc-local-data", "config", "default.yml")); os.IsNotExist(err) {
		t.Errorf("expected config/default.yml to exist")
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
		filepath.Join(".ohc-cloud-data", "telemetry"),
	}

	for _, dir := range expectedDirs {
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			t.Errorf("expected directory %s to exist", dir)
		}
	}

	if _, err := os.Stat(filepath.Join(".ohc-cloud-data", "config", "default.yml")); os.IsNotExist(err) {
		t.Errorf("expected config/default.yml to exist")
	}

	os.RemoveAll(".ohc-cloud-data")
}

func TestCheckEnvironment_Local(t *testing.T) {
	// Ensure clean state
	os.RemoveAll(".ohc-local-data")

	err := CheckEnvironment(false)
	if err == nil {
		t.Fatalf("expected error for missing environment, got nil")
	}

	ProvisionEnvironment(context.Background(), false)
	err = CheckEnvironment(false)
	if err != nil {
		t.Fatalf("expected nil error for provisioned environment, got %v", err)
	}
	os.RemoveAll(".ohc-local-data")
}

func TestCheckEnvironment_Cloud(t *testing.T) {
	// Ensure clean state
	os.RemoveAll(".ohc-cloud-data")

	err := CheckEnvironment(true)
	if err == nil {
		t.Fatalf("expected error for missing environment, got nil")
	}

	ProvisionEnvironment(context.Background(), true)
	err = CheckEnvironment(true)
	if err != nil {
		t.Fatalf("expected nil error for provisioned environment, got %v", err)
	}
	os.RemoveAll(".ohc-cloud-data")
}
