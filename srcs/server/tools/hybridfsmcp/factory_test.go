package hybridfsmcp

import (
	"testing"
)

func TestNewProviderFactory_CloudMode(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	t.Setenv("OHC_CLOUD_FS_DIR", "/tmp/cloudfs")

	provider := NewProviderFactory()
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("expected *CloudFSProvider, got %T", provider)
	}
}

func TestNewProviderFactory_StandaloneMode(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "")
	t.Setenv("OHC_LOCAL_FS_DIR", "/tmp/localfs")

	provider := NewProviderFactory()
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("expected *LocalFSProvider, got %T", provider)
	}
}

func TestNewHybridFSMCPFromEnv(t *testing.T) {
	mcp := NewHybridFSMCPFromEnv()
	if mcp == nil {
		t.Errorf("expected non-nil MCP")
	}
	if len(mcp.ListTools()) != 3 {
		t.Errorf("expected 3 tools, got %d", len(mcp.ListTools()))
	}
}
