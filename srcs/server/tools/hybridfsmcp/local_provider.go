package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

// LocalFSProvider implements FileSystemProvider for the local filesystem.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to a base directory.
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBaseDir, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{
		baseDir: absBaseDir,
	}, nil
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	cleanPath := filepath.Clean(path)
    // Avoid double-prefixing if already absolute under baseDir
    var fullPath string
    if filepath.IsAbs(cleanPath) {
        fullPath = cleanPath
    } else {
        fullPath = filepath.Join(p.baseDir, cleanPath)
    }

	absPath, err := filepath.Abs(fullPath)
	if err != nil {
		return "", err
	}

    // Boundary check with trailing separator to prevent partial name match vulnerability
    baseDirWithSep := p.baseDir
    if !strings.HasSuffix(baseDirWithSep, string(filepath.Separator)) {
        baseDirWithSep += string(filepath.Separator)
    }

	if !strings.HasPrefix(absPath, baseDirWithSep) && absPath != p.baseDir {
		return "", errors.New("path escapes base directory")
	}

	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var fileInfos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		fileInfos = append(fileInfos, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}

	return fileInfos, nil
}
