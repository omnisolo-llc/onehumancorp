package hybridfsmcp

import (
	"fmt"
	"os"
)

func NewProvider() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" || os.Getenv("OHC_STANDALONE") != "true" {
		baseDir := os.Getenv("MCP_BUNDLE_DIR")
		if baseDir == "" {
			baseDir = "/tmp/cloudfs"
		}
		return NewCloudFSProvider(baseDir)
	}

	workspaceDir := os.Getenv("WORKSPACE_DIR")
	if workspaceDir == "" {
		workspaceDir = "/tmp/localfs"
	}
	return NewLocalFSProvider(workspaceDir)
}

// Ensure factory logic correctly instantiates the provider based on the OHC_MULTITENANT and OHC_STANDALONE modes.
func Serve() {
	fmt.Println("Starting Hybrid FS MCP Server")
}
