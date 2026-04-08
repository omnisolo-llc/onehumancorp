package mcp

import (
	"fmt"
	"os"
)

// NewFileSystemProvider creates the correct FileSystemProvider based on the environment.
// It checks OHC_MULTITENANT and OHC_STANDALONE.
func NewFileSystemProvider() (FileSystemProvider, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		baseMount := os.Getenv("OHC_FS_MOUNT")
		if baseMount == "" {
			// default fallback for testing/dev
			baseMount = "/tmp/ohc-tenants"
		}
		return NewCloudFSProvider(baseMount)
	}

	if os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("OHC_STANDALONE") == "" {
		workspaceDir := os.Getenv("OHC_WORKSPACE_DIR")
		if workspaceDir == "" {
			// default to current working directory
			cwd, err := os.Getwd()
			if err != nil {
				return nil, fmt.Errorf("failed to get current working directory: %w", err)
			}
			workspaceDir = cwd
		}
		return NewLocalFSProvider(workspaceDir)
	}

	return nil, fmt.Errorf("could not determine appropriate FileSystemProvider based on environment variables")
}
