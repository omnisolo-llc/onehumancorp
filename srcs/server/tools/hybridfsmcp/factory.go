package hybridfsmcp

func NewFileSystemProvider(isStandalone bool, baseDir string) FileSystemProvider {
	if isStandalone {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}
