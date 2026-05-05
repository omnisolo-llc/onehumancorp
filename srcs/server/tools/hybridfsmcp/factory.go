package hybridfsmcp

import (
	"os"
)

// NewProvider creates a FileSystemProvider based on environment variables.
// It checks OHC_MULTITENANT and OHC_STANDALONE to determine if it should
// return a CloudFSProvider or LocalFSProvider.
func NewProvider() FileSystemProvider {
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"

	if isMultiTenant {
		baseCloudDir := os.Getenv("OHC_BASE_CLOUD_DIR")
		if baseCloudDir == "" {
			baseCloudDir = "/mnt/cloud_data" // Default fallback
		}
		return &CloudFSProvider{
			BaseCloudDir: baseCloudDir,
		}
	}

	// Default to Local/Standalone
	workspaceDir := os.Getenv("OHC_WORKSPACE_DIR")
	if workspaceDir == "" {
		workspaceDir = "./workspace" // Default fallback
	}
	return &LocalFSProvider{
		WorkspaceDir: workspaceDir,
	}
}
