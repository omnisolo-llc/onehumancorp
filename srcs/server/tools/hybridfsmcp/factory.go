package hybridfsmcp

import (
	"os"
)

// NewProvider creates a FileSystemProvider based on environment variables.
func NewProvider() FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("OHC_MULTITENANT") != "true" {
		workspace := os.Getenv("OHC_WORKSPACE_DIR")
		if workspace == "" {
			workspace = "/tmp/ohc_workspace"
		}
		return NewLocalFSProvider(workspace)
	}

	cloudBase := os.Getenv("OHC_CLOUD_FS_BASE")
	if cloudBase == "" {
		cloudBase = "/mnt/tenant_volumes"
	}
	return NewCloudFSProvider(cloudBase)
}
