package hybridfsmcp

import (
	"os"
	"testing"
)

func TestNewProviderFromEnv(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "")
	os.Setenv("OHC_STANDALONE", "true")

	provider, err := NewProviderFromEnv("/tmp/workspace", "/tmp/cloud")
	if err != nil {
		t.Fatal(err)
	}

	if !provider.IsLocal() {
		t.Errorf("expected IsLocal to be true when OHC_MULTITENANT is empty")
	}

	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_STANDALONE", "")

	provider, err = NewProviderFromEnv("/tmp/workspace", "/tmp/cloud")
	if err != nil {
		t.Fatal(err)
	}

	if provider.IsLocal() {
		t.Errorf("expected IsLocal to be false when OHC_MULTITENANT is true")
	}

	// Clean up
	os.Setenv("OHC_MULTITENANT", "")
	os.Setenv("OHC_STANDALONE", "")
}
