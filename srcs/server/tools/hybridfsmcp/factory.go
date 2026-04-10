package hybridfsmcp

import (
	"os"
)

// NewProviderFromEnv instantiates the correct FileSystemProvider based on environment variables.
func NewProviderFromEnv(workspaceDir, cloudBaseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(cloudBaseDir)
	}
	// Default to standalone/local mode
	return NewLocalFSProvider(workspaceDir)
}
