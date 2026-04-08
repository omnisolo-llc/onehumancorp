package hybridfsmcp

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
)

// FileSystemProvider defines the interface for interacting with the hybrid file system.
type FileSystemProvider interface {
	ReadFile(path string) ([]byte, error)
	WriteFile(path string, content []byte) error
	ListDir(path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, restricting access to a base directory.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	// ensure directory exists
	if err := os.MkdirAll(absBase, 0755); err != nil {
		return nil, err
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.baseDir, targetPath))
	if err != nil {
		return "", err
	}

	rel, err := filepath.Rel(p.baseDir, absPath)
	if err != nil {
		return "", err
	}

	rel = filepath.ToSlash(rel)
	if rel == ".." || strings.HasPrefix(rel, "../") {
		return "", errors.New("access denied: path escapes base directory")
	}

	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(path string, content []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, content, 0644)
}

func (p *LocalFSProvider) ListDir(path string) ([]string, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, namespacing by organization_id.
type CloudFSProvider struct {
	baseDir        string
	organizationID string
}

func NewCloudFSProvider(baseDir, organizationID string) (*CloudFSProvider, error) {
	if organizationID == "" {
		return nil, errors.New("organizationID is required for CloudFSProvider")
	}

	absBase, err := filepath.Abs(filepath.Join(baseDir, organizationID))
	if err != nil {
		return nil, err
	}

	// ensure directory exists
	if err := os.MkdirAll(absBase, 0755); err != nil {
		return nil, err
	}

	return &CloudFSProvider{
		baseDir:        absBase,
		organizationID: organizationID,
	}, nil
}

func (p *CloudFSProvider) resolvePath(targetPath string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.baseDir, targetPath))
	if err != nil {
		return "", err
	}

	rel, err := filepath.Rel(p.baseDir, absPath)
	if err != nil {
		return "", err
	}

	rel = filepath.ToSlash(rel)
	if rel == ".." || strings.HasPrefix(rel, "../") {
		return "", errors.New("access denied: path escapes tenant directory")
	}

	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(path string, content []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, content, 0644)
}

func (p *CloudFSProvider) ListDir(path string) ([]string, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var files []string
	for _, entry := range entries {
		files = append(files, entry.Name())
	}
	return files, nil
}

func NewFileSystemProvider(isStandalone bool, baseDir string, organizationID string) (FileSystemProvider, error) {
	if isStandalone {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir, organizationID)
}
