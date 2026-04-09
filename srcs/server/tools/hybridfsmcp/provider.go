package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *LocalFSProvider) checkPath(pPath string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, pPath))
	if cleanPath == p.baseDir || strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return cleanPath, nil
	}
	return "", errors.New("access denied: path outside base directory")
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	cleanPath, err := p.checkPath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(cleanPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	cleanPath, err := p.checkPath(path)
	if err != nil {
		return err
	}
	return os.WriteFile(cleanPath, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	cleanPath, err := p.checkPath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(cleanPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

type CloudFSProvider struct{}

func NewCloudFSProvider() *CloudFSProvider {
	return &CloudFSProvider{}
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	return nil, errors.New("unimplemented cloud read")
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	return errors.New("unimplemented cloud write")
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	return nil, errors.New("unimplemented cloud list")
}
