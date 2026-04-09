package hybridfsmcp

import (
	"os"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// NewHybridFSProvider creates a FileSystemProvider based on the current OHC mode.
func NewHybridFSProvider() (mcp.FileSystemProvider, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		// Cloud mode
		basePath := os.Getenv("OHC_CLOUD_FS_BASE_PATH")
		if basePath == "" {
			basePath = filepath.Join(os.TempDir(), "ohc_cloud_fs")
		}
		return mcp.NewCloudFSProvider(basePath)
	}

	// Default to standalone mode
	basePath := os.Getenv("OHC_LOCAL_FS_BASE_PATH")
	if basePath == "" {
		basePath = filepath.Join(os.TempDir(), "ohc_local_fs")
	}
	return mcp.NewLocalFSProvider(basePath)
}
