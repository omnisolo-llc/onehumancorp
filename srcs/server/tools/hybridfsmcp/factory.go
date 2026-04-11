package hybridfsmcp

import (
	"os"
	"path/filepath"
)

// NewHybridFSMCP creates a new HybridFSMCP, instantiating the appropriate FileSystemProvider
// depending on whether the system is in Cloud mode (multi-tenant) or Standalone mode.
func NewHybridFSMCP(baseDir string) *HybridFSMCP {
	// The OHC memory states OHC_MULTITENANT dictates the mode
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"

	absBaseDir, err := filepath.Abs(baseDir)
	if err == nil {
		baseDir = absBaseDir
	}

	var provider FileSystemProvider
	if isMultiTenant {
		provider = &CloudFSProvider{BaseDir: baseDir}
	} else {
		provider = &LocalFSProvider{BaseDir: baseDir}
	}

	return &HybridFSMCP{
		provider: provider,
	}
}
