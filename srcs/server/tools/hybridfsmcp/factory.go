package hybridfsmcp

import (
	"os"
)

// NewHybridFSMCP creates a new HybridFSMCP server instance appropriately configured for the environment.
// Returns an MCP server using either the LocalFSProvider or CloudFSProvider based on OHC_STANDALONE.
func NewHybridFSMCP(baseWorkspace string) *HybridFSMCP {
	var provider FileSystemProvider

	if os.Getenv("OHC_STANDALONE") == "true" {
		provider = NewLocalFSProvider(baseWorkspace)
	} else {
		provider = NewCloudFSProvider(baseWorkspace)
	}

	return NewHybridFSMCPServer(provider)
}
