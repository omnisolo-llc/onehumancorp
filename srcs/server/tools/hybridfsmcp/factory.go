package hybridfsmcp

import (
	"os"
)

// NewFileSystemProvider creates the appropriate FileSystemProvider based on the current mode.
func NewFileSystemProvider(localWorkspaceDir string, cloudMountPoint string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(cloudMountPoint)
	}
	// Default to standalone/local mode
	return NewLocalFSProvider(localWorkspaceDir)
}
