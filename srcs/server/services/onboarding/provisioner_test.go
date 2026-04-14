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

	expectedConfigFile := filepath.Join(".ohc-local-data", "config", "ohc.yaml")
	if _, err := os.Stat(expectedConfigFile); os.IsNotExist(err) {
		t.Errorf("expected config file %s to exist", expectedConfigFile)
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

	expectedConfigFile := filepath.Join(".ohc-cloud-data", "config", "ohc.yaml")
	if _, err := os.Stat(expectedConfigFile); os.IsNotExist(err) {
		t.Errorf("expected config file %s to exist", expectedConfigFile)
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

func TestCleanupEnvironment_Local(t *testing.T) {
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
	ProvisionEnvironment(context.Background(), true)
	err := CleanupEnvironment(context.Background(), true)
	if err != nil {
		t.Fatalf("expected nil error for cleanup environment, got %v", err)
	}
	if err := CheckEnvironment(true); err == nil {
		t.Fatalf("expected error for missing environment after cleanup, got nil")
	}
}

func TestValidateEnvironment_Local(t *testing.T) {
	os.RemoveAll(".ohc-local-data")

	// Should fail before provisioning
	if err := ValidateEnvironment(context.Background(), false); err == nil {
		t.Fatalf("expected error validating unprovisioned environment, got nil")
	}

	ProvisionEnvironment(context.Background(), false)
	if err := ValidateEnvironment(context.Background(), false); err != nil {
		t.Fatalf("expected nil error validating provisioned environment, got %v", err)
	}
	os.RemoveAll(".ohc-local-data")
}

func TestValidateEnvironment_Cloud(t *testing.T) {
	os.RemoveAll(".ohc-cloud-data")

	// Should fail before provisioning
	if err := ValidateEnvironment(context.Background(), true); err == nil {
		t.Fatalf("expected error validating unprovisioned environment, got nil")
	}

	ProvisionEnvironment(context.Background(), true)
	if err := ValidateEnvironment(context.Background(), true); err != nil {
		t.Fatalf("expected nil error validating provisioned environment, got %v", err)
	}
	os.RemoveAll(".ohc-cloud-data")
}
