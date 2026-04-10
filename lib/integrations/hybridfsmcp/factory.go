package hybridfsmcp

import (
    "os"
)

func NewFileSystemProvider() FileSystemProvider {
    isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"

    cloudPath := os.Getenv("OHC_CLOUD_FS_PATH")
    if cloudPath == "" {
        cloudPath = "."
    }

    localPath := os.Getenv("OHC_LOCAL_FS_PATH")
    if localPath == "" {
        localPath = "."
    }

    if isMultiTenant {
        return NewCloudFSProvider(cloudPath)
    }
    return NewLocalFSProvider(localPath)
}

func NewFactoryHybridFSProxyMCP() *HybridFSProxyMCP {
    provider := NewFileSystemProvider()
    return NewHybridFSProxyMCP(provider)
}
