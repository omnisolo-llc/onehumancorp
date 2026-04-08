package hybridfsmcp

import (
	"os"
)

// NewProvider creates the appropriate FileSystemProvider based on the environment.
func NewProvider() FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		workspaceDir := os.Getenv("OHC_WORKSPACE_DIR")
		if workspaceDir == "" {
			workspaceDir = "./workspace"
		}
		return NewLocalFSProvider(workspaceDir)
	}

	baseVolumeDir := os.Getenv("OHC_CLOUD_VOLUME_DIR")
	if baseVolumeDir == "" {
		baseVolumeDir = "/mnt/data/tenants"
	}
	return NewCloudFSProvider(baseVolumeDir)
}
