package restic

import (
	"context"
	"os"
	"testing"
)

func TestProviderCloudMode(t *testing.T) {
	os.Setenv("OHC_EXECUTION_MODE", "cloud")
	defer os.Unsetenv("OHC_EXECUTION_MODE")

	p := NewProvider()

	if p.Name() != "restic" {
		t.Errorf("Expected name restic, got %s", p.Name())
	}

	if p.Status() != "unsupported" {
		t.Errorf("Expected status unsupported, got %s", p.Status())
	}

	_, err := p.ResticSnapshot(context.Background(), "/tmp/repo", "password", []string{"/tmp/data"})
	if err == nil {
		t.Errorf("Expected error in cloud mode")
	}

	_, err = p.ResticRestore(context.Background(), "/tmp/repo", "password", "latest", "/tmp/target")
	if err == nil {
		t.Errorf("Expected error in cloud mode")
	}

	_, err = p.ResticStatus(context.Background(), "/tmp/repo", "password")
	if err == nil {
		t.Errorf("Expected error in cloud mode")
	}
}

func TestProviderStandaloneMode(t *testing.T) {
	os.Setenv("OHC_EXECUTION_MODE", "standalone")
	defer os.Unsetenv("OHC_EXECUTION_MODE")

	p := NewProvider()

	if p.Status() != "active" {
		t.Errorf("Expected status active, got %s", p.Status())
	}
}
