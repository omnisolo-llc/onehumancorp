package mcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error)
}

type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	absPath, err := filepath.Abs(baseDir)
	if err != nil {
		absPath = baseDir
	}
	return &LocalFSProvider{baseDir: absPath}
}

func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	fullPath := filepath.Join(p.baseDir, reqPath)
	cleanPath := filepath.Clean(fullPath)

	baseDirWithSep := p.baseDir
	if !strings.HasSuffix(baseDirWithSep, string(filepath.Separator)) {
		baseDirWithSep += string(filepath.Separator)
	}

	if !strings.HasPrefix(cleanPath+string(filepath.Separator), baseDirWithSep) {
		return "", errors.New("path escapes base directory")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	safeDir, err := p.resolvePath(dir)
	if err != nil {
		return nil, err
	}
	var matches []string
	err = filepath.WalkDir(safeDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			rel, _ := filepath.Rel(p.baseDir, path)
			matches = append(matches, rel)
		}
		return nil
	})
	return matches, err
}

type CloudFSProvider struct {
	tenantID string
	memoryDB map[string][]byte
}

func NewCloudFSProvider(tenantID string) *CloudFSProvider {
	return &CloudFSProvider{
		tenantID: tenantID,
		memoryDB: make(map[string][]byte),
	}
}

func (p *CloudFSProvider) resolvePath(reqPath string) string {
	return filepath.Join(p.tenantID, reqPath)
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	key := p.resolvePath(path)
	data, ok := p.memoryDB[key]
	if !ok {
		return nil, os.ErrNotExist
	}
	return data, nil
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	key := p.resolvePath(path)
	p.memoryDB[key] = append([]byte(nil), data...)
	return nil
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	prefix := p.resolvePath(path)
	if !strings.HasSuffix(prefix, "/") {
		prefix += "/"
	}
	var names []string
	seen := make(map[string]bool)
	for k := range p.memoryDB {
		if strings.HasPrefix(k, prefix) {
			rest := strings.TrimPrefix(k, prefix)
			parts := strings.SplitN(rest, "/", 2)
			if len(parts) > 0 && !seen[parts[0]] {
				seen[parts[0]] = true
				names = append(names, parts[0])
			}
		}
	}
	return names, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	prefix := p.resolvePath(dir)
	var matches []string
	for k := range p.memoryDB {
		if strings.HasPrefix(k, prefix) && strings.Contains(k, pattern) {
			rel := strings.TrimPrefix(k, p.tenantID+"/")
			matches = append(matches, rel)
		}
	}
	return matches, nil
}
