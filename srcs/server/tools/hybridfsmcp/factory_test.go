package hybridfsmcp

import (
	"os"
	"testing"
)

func TestNewProvider(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	provider := NewProvider()
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Fatalf("Expected CloudFSProvider")
	}

	os.Setenv("OHC_MULTITENANT", "false")
	provider = NewProvider()
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Fatalf("Expected LocalFSProvider")
	}
}
