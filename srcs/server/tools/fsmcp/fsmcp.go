package fsmcp

import (
	"context"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/minio/minio-go/v7"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FsMcpTool struct {
	isStandalone   bool
	localBaseDir   string
	s3Client       *minio.Client
	s3Bucket       string
}

func NewFsMcpTool(s3Client *minio.Client, s3Bucket string) *FsMcpTool {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	var localBaseDir string
	if home, err := os.UserHomeDir(); err == nil {
		localBaseDir = filepath.Join(home, ".ohc-local-data", "fs")
	} else {
		localBaseDir = filepath.Join("/tmp", ".ohc-local-data", "fs")
	}

	return &FsMcpTool{
		isStandalone: isStandalone,
		localBaseDir: localBaseDir,
		s3Client:     s3Client,
		s3Bucket:     s3Bucket,
	}
}

func (t *FsMcpTool) cleanPath(claims *auth.Claims, requestedPath string) (string, error) {
	cleanPath := filepath.Clean(requestedPath)
	if strings.Contains(cleanPath, "..") || strings.HasPrefix(cleanPath, "/") {
		return "", fmt.Errorf("invalid path: path traversal detected")
	}

	if t.isStandalone {
		fullPath := filepath.Join(t.localBaseDir, cleanPath)
		if !strings.HasPrefix(fullPath, t.localBaseDir) {
			return "", fmt.Errorf("invalid path: outside base directory")
		}
		return fullPath, nil
	}

	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("organization ID required for cloud mode")
	}

	s3Key := fmt.Sprintf("tenant/%s/fs/%s", claims.OrganizationID, cleanPath)
	return s3Key, nil
}

func (t *FsMcpTool) Read(ctx context.Context, claims *auth.Claims, path string) (string, error) {
	resolvedPath, err := t.cleanPath(claims, path)
	if err != nil {
		return "", err
	}

	if t.isStandalone {
		data, err := os.ReadFile(resolvedPath)
		if err != nil {
			return "", err
		}
		return string(data), nil
	}

	if t.s3Client == nil {
		return "", fmt.Errorf("s3 client not configured")
	}

	obj, err := t.s3Client.GetObject(ctx, t.s3Bucket, resolvedPath, minio.GetObjectOptions{})
	if err != nil {
		return "", err
	}
	defer obj.Close()

	data, err := io.ReadAll(obj)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func (t *FsMcpTool) Write(ctx context.Context, claims *auth.Claims, path string, content string) error {
	resolvedPath, err := t.cleanPath(claims, path)
	if err != nil {
		return err
	}

	if t.isStandalone {
		dir := filepath.Dir(resolvedPath)
		if err := os.MkdirAll(dir, 0755); err != nil {
			return err
		}
		return os.WriteFile(resolvedPath, []byte(content), 0644)
	}

	if t.s3Client == nil {
		return fmt.Errorf("s3 client not configured")
	}

	reader := strings.NewReader(content)
	_, err = t.s3Client.PutObject(ctx, t.s3Bucket, resolvedPath, reader, reader.Size(), minio.PutObjectOptions{
		ContentType: "application/octet-stream",
	})
	return err
}

func (t *FsMcpTool) List(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	cleanPath := filepath.Clean(path)
	if strings.Contains(cleanPath, "..") || strings.HasPrefix(cleanPath, "/") {
		return nil, fmt.Errorf("invalid path: path traversal detected")
	}

	if t.isStandalone {
		fullPath := filepath.Join(t.localBaseDir, cleanPath)
		if !strings.HasPrefix(fullPath, t.localBaseDir) {
			return nil, fmt.Errorf("invalid path: outside base directory")
		}

		var entries []string
		files, err := os.ReadDir(fullPath)
		if err != nil {
			if os.IsNotExist(err) {
				return []string{}, nil
			}
			return nil, err
		}

		for _, f := range files {
			entries = append(entries, f.Name())
		}
		return entries, nil
	}

	if claims == nil || claims.OrganizationID == "" {
		return nil, fmt.Errorf("organization ID required for cloud mode")
	}

	prefix := fmt.Sprintf("tenant/%s/fs/%s", claims.OrganizationID, cleanPath)
	if prefix != "" && !strings.HasSuffix(prefix, "/") {
		prefix += "/"
	}

	if t.s3Client == nil {
		return nil, fmt.Errorf("s3 client not configured")
	}

	var entries []string
	opts := minio.ListObjectsOptions{
		Prefix:    prefix,
		Recursive: false,
	}

	for object := range t.s3Client.ListObjects(ctx, t.s3Bucket, opts) {
		if object.Err != nil {
			return nil, object.Err
		}

		key := strings.TrimPrefix(object.Key, prefix)
		if key != "" {
			entries = append(entries, key)
		}
	}

	return entries, nil
}
