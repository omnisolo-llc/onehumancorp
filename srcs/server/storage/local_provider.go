package storage

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

// LocalProvider implements Provider for the local filesystem.
type LocalProvider struct {
	basePath string
}

// NewLocalProvider creates a new LocalProvider with the given base directory.
func NewLocalProvider(basePath string) (*LocalProvider, error) {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	if err := os.MkdirAll(absPath, 0755); err != nil {
		return nil, err
	}
	return &LocalProvider{basePath: absPath}, nil
}

func (p *LocalProvider) IsLocal() bool {
	return true
}

func (p *LocalProvider) getLocalPath(key string) string {
	cleanPath := filepath.Clean(filepath.Join(p.basePath, key))
	if cleanPath != p.basePath && !strings.HasPrefix(cleanPath, p.basePath+string(filepath.Separator)) {
		// Prevent directory traversal
		return p.basePath
	}
	return cleanPath
}

func (p *LocalProvider) ListBlobs(ctx context.Context, prefix string) ([]BlobMetadata, error) {
	var blobs []BlobMetadata
	searchDir := p.getLocalPath(prefix)

	// If prefix doesn't end with '/', it could be part of a filename or a directory.
	// For simplicity in Local FS, let's just search the directory if it's one,
	// or search the parent directory and filter by prefix.
	parentDir := filepath.Dir(searchDir)

	dirToWalk := searchDir
	info, err := os.Stat(searchDir)
	if err != nil {
		if os.IsNotExist(err) {
			// Might be a file prefix, walk the parent
			dirToWalk = parentDir
			if dirToWalk == p.basePath {
				// Don't modify basePrefix
			}
		} else {
			return nil, err
		}
	} else if !info.IsDir() {
		dirToWalk = parentDir
	}

	err = filepath.WalkDir(dirToWalk, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			return nil
		}

		relPath, err := filepath.Rel(p.basePath, path)
		if err != nil {
			return nil // Skip if can't make relative
		}

		// Force forward slashes for keys
		key := filepath.ToSlash(relPath)

		if !strings.HasPrefix(key, prefix) {
			return nil // Skip if it doesn't match the requested prefix
		}

		info, err := d.Info()
		if err != nil {
			return nil // Skip if we can't get info
		}

		blobs = append(blobs, BlobMetadata{
			Key:          key,
			Size:         info.Size(),
			LastModified: info.ModTime(),
			ContentType:  "application/octet-stream", // Fallback, could sniff
		})
		return nil
	})

	return blobs, err
}

func (p *LocalProvider) ReadBlobMetadata(ctx context.Context, key string) (BlobMetadata, error) {
	path := p.getLocalPath(key)
	info, err := os.Stat(path)
	if err != nil {
		return BlobMetadata{}, fmt.Errorf("read metadata: %w", err)
	}
	if info.IsDir() {
		return BlobMetadata{}, fmt.Errorf("key is a directory")
	}

	return BlobMetadata{
		Key:          key,
		Size:         info.Size(),
		LastModified: info.ModTime(),
		ContentType:  "application/octet-stream",
	}, nil
}

func (p *LocalProvider) GetBlobURL(ctx context.Context, key string) (string, error) {
	// For local provider, we might just return the file:// URL or a dummy local URL.
	// Since MCP tools are run locally, a local absolute path is fine.
	path := p.getLocalPath(key)
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return "", fmt.Errorf("blob does not exist")
	}
	return "file://" + path, nil
}
