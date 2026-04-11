package hybridfsmcp

import "os"

func NewProvider() FileSystemProvider {
	basePath := "/tmp/hybridfs"
	if os.Getenv("OHC_STANDALONE") != "true" {
		return NewCloudFSProvider(basePath)
	}
	return NewLocalFSProvider(basePath)
}
