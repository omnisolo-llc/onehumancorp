package mcp

// NewFileSystemProvider creates a FileSystemProvider based on the provided mode flags.
func NewFileSystemProvider(isMultiTenant bool, baseDir string) FileSystemProvider {
	if isMultiTenant {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}
