package hybridfsmcp

import (
	"os"
)

// NewProvider creates the appropriate FileSystemProvider based on the environment configuration.
func NewProvider() FileSystemProvider {
	baseDir := os.Getenv("OHC_FS_ROOT")
	if baseDir == "" {
		baseDir = "/tmp/ohc_fs" // Fallback directory
	}

	isMultitenant := os.Getenv("OHC_MULTITENANT") == "true"
	if isMultitenant {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}
