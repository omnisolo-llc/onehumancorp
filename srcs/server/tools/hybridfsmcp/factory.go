package hybridfsmcp

import (
	"os"
	"path/filepath"
)

// NewProviderFactory creates a FileSystemProvider based on the current mode.
func NewProviderFactory() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		// Cloud mode
		baseDir := os.Getenv("OHC_CLOUD_FS_DIR")
		if baseDir == "" {
			baseDir = "/var/lib/ohc/tenant-fs"
		}
		return NewCloudFSProvider(baseDir)
	}

	// Standalone/Local mode
	baseDir := os.Getenv("OHC_LOCAL_FS_DIR")
	if baseDir == "" {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			baseDir = "/tmp/ohc-workspace"
		} else {
			baseDir = filepath.Join(homeDir, ".ohc", "workspace")
		}
	}
	return NewLocalFSProvider(baseDir)
}

// NewHybridFSMCPFromEnv creates an MCP configured via the environment.
func NewHybridFSMCPFromEnv() *HybridFSMCP {
	return NewHybridFSMCP(NewProviderFactory())
}
