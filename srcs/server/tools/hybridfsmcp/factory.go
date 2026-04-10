package hybridfsmcp

import "os"

func NewProvider() FileSystemProvider {
	fsRoot := os.Getenv("OHC_FS_ROOT")
	if fsRoot == "" {
		fsRoot = os.TempDir()
	}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(fsRoot)
	}
	return NewLocalFSProvider(fsRoot)
}
