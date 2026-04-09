package mcp

import (
	"os"
)

// NewFileSystemProvider creates the appropriate FileSystemProvider based on environment configuration.
func NewFileSystemProvider(basePath string) (FileSystemProvider, error) {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	// OHC_MULTITENANT usually implies cloud mode, but we can just use STANDALONE as the primary toggle

	if isStandalone {
		return NewLocalFSProvider(basePath)
	}

	return NewCloudFSProvider(basePath)
}
