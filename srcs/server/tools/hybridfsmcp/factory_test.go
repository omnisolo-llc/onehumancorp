package hybridfsmcp

import (
	"testing"
)

func TestNewProvider_Local(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_MULTITENANT", "")

	tempDir := t.TempDir()
	provider, err := NewProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	if !provider.IsLocal() {
		t.Error("expected local provider when OHC_STANDALONE=true")
	}
}

func TestNewProvider_Cloud(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "")
	t.Setenv("OHC_MULTITENANT", "true")

	tempDir := t.TempDir()
	provider, err := NewProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	if provider.IsLocal() {
		t.Error("expected cloud provider when OHC_MULTITENANT=true")
	}
}
