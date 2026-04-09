package hybridfsmcp

import (
    "os"
)

func NewProvider() FileSystemProvider {
    if os.Getenv("OHC_MULTITENANT") == "true" {
        rootVolume := os.Getenv("OHC_FS_ROOT")
        if rootVolume == "" {
            rootVolume = "/data/ohc-fs"
        }
        return NewCloudFSProvider(rootVolume)
    }

    // Default to standalone/local mode
    baseDir := os.Getenv("OHC_FS_ROOT")
    if baseDir == "" {
        baseDir = "./workspace"
    }
    return NewLocalFSProvider(baseDir)
}
