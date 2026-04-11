package hybridfsmcp

import (
	"fmt"
	"os"
)

// NewProviderFactory creates a FileSystemProvider based on the current mode (Multi-tenant vs Standalone)
func NewProviderFactory(baseDir string) (FileSystemProvider, error) {
	// memory rule: rely strictly on evaluating OHC_MULTITENANT to distinguish between Multi-tenant and Standalone modes.
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"

	if isMultiTenant {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}

// NewServerFactory creates an MCP Server with the appropriate provider based on the current mode
func NewServerFactory(baseDir string) (*Server, error) {
	provider, err := NewProviderFactory(baseDir)
	if err != nil {
		return nil, fmt.Errorf("failed to initialize provider: %w", err)
	}
	return NewServer(provider), nil
}
