package storage_test

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/storage"
)

// newTestProvider creates a LocalProvider backed by a temporary directory.
func newTestProvider(t *testing.T) *storage.LocalProvider {
	t.Helper()
	dir := t.TempDir()
	p, err := storage.NewLocalProvider(dir)
	if err != nil {
		t.Fatalf("NewLocalProvider: %v", err)
	}
	return p
}

// writeFile writes content to a file inside the provider's base directory for
// test setup without going through the provider API.
func writeFile(t *testing.T, dir, key, content string) {
	t.Helper()
	full := filepath.Join(dir, filepath.FromSlash(key))
	if err := os.MkdirAll(filepath.Dir(full), 0755); err != nil {
		t.Fatalf("writeFile MkdirAll: %v", err)
	}
	if err := os.WriteFile(full, []byte(content), 0644); err != nil {
		t.Fatalf("writeFile WriteFile: %v", err)
	}
}

func TestLocalProvider_IsLocal(t *testing.T) {
	p := newTestProvider(t)
	if !p.IsLocal() {
		t.Error("expected IsLocal() == true for LocalProvider")
	}
}

func TestLocalProvider_ListBlobs_Empty(t *testing.T) {
	p := newTestProvider(t)
	blobs, err := p.ListBlobs(context.Background(), "")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(blobs) != 0 {
		t.Errorf("expected 0 blobs, got %d", len(blobs))
	}
}

func TestLocalProvider_ListBlobs_SingleFile(t *testing.T) {
	dir := t.TempDir()
	p, _ := storage.NewLocalProvider(dir)
	writeFile(t, dir, "foo/bar.txt", "hello")

	blobs, err := p.ListBlobs(context.Background(), "foo/")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(blobs) != 1 {
		t.Fatalf("expected 1 blob, got %d", len(blobs))
	}
	if blobs[0].Key != "foo/bar.txt" {
		t.Errorf("unexpected key: %q", blobs[0].Key)
	}
	if blobs[0].Size != int64(len("hello")) {
		t.Errorf("expected size %d, got %d", len("hello"), blobs[0].Size)
	}
	if blobs[0].LastModified.IsZero() {
		t.Error("expected non-zero LastModified")
	}
}

func TestLocalProvider_ListBlobs_MultipleFiles(t *testing.T) {
	dir := t.TempDir()
	p, _ := storage.NewLocalProvider(dir)
	writeFile(t, dir, "data/a.txt", "aaa")
	writeFile(t, dir, "data/b.txt", "bbbb")
	writeFile(t, dir, "other/c.txt", "ccccc")

	blobs, err := p.ListBlobs(context.Background(), "data/")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(blobs) != 2 {
		t.Fatalf("expected 2 blobs under data/, got %d", len(blobs))
	}
}

func TestLocalProvider_ListBlobs_AllFiles(t *testing.T) {
	dir := t.TempDir()
	p, _ := storage.NewLocalProvider(dir)
	writeFile(t, dir, "x.bin", "x")
	writeFile(t, dir, "sub/y.bin", "yy")

	blobs, err := p.ListBlobs(context.Background(), "")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(blobs) < 2 {
		t.Errorf("expected at least 2 blobs, got %d", len(blobs))
	}
}

func TestLocalProvider_ReadBlobMetadata_ExistingFile(t *testing.T) {
	dir := t.TempDir()
	p, _ := storage.NewLocalProvider(dir)
	content := "test content"
	writeFile(t, dir, "docs/readme.md", content)

	meta, err := p.ReadBlobMetadata(context.Background(), "docs/readme.md")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if meta.Key != "docs/readme.md" {
		t.Errorf("expected key 'docs/readme.md', got %q", meta.Key)
	}
	if meta.Size != int64(len(content)) {
		t.Errorf("expected size %d, got %d", len(content), meta.Size)
	}
	if meta.LastModified.IsZero() {
		t.Error("expected non-zero LastModified")
	}
	if meta.LastModified.After(time.Now().Add(time.Second)) {
		t.Error("LastModified is in the future")
	}
}

func TestLocalProvider_ReadBlobMetadata_MissingFile(t *testing.T) {
	p := newTestProvider(t)
	_, err := p.ReadBlobMetadata(context.Background(), "nonexistent/file.txt")
	if err == nil {
		t.Error("expected error for missing file, got nil")
	}
}

func TestLocalProvider_ReadBlobMetadata_Directory(t *testing.T) {
	dir := t.TempDir()
	p, _ := storage.NewLocalProvider(dir)
	// Create a sub-directory (not a file)
	if err := os.MkdirAll(filepath.Join(dir, "subdir"), 0755); err != nil {
		t.Fatal(err)
	}
	_, err := p.ReadBlobMetadata(context.Background(), "subdir")
	if err == nil {
		t.Error("expected error when key is a directory, got nil")
	}
}

func TestLocalProvider_GetBlobURL_ExistingFile(t *testing.T) {
	dir := t.TempDir()
	p, _ := storage.NewLocalProvider(dir)
	writeFile(t, dir, "assets/logo.png", "png-data")

	url, err := p.GetBlobURL(context.Background(), "assets/logo.png")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if url == "" {
		t.Error("expected non-empty URL")
	}
	// Should be a file:// URL
	if len(url) < 7 || url[:7] != "file://" {
		t.Errorf("expected file:// URL, got %q", url)
	}
}

func TestLocalProvider_GetBlobURL_MissingFile(t *testing.T) {
	p := newTestProvider(t)
	_, err := p.GetBlobURL(context.Background(), "missing.txt")
	if err == nil {
		t.Error("expected error for missing blob, got nil")
	}
}

func TestLocalProvider_PathTraversal_ListBlobs(t *testing.T) {
	p := newTestProvider(t)
	// Attempt directory traversal
	blobs, err := p.ListBlobs(context.Background(), "../../../etc/")
	// Either returns an error or returns 0 blobs - both are acceptable safe behaviours.
	if err == nil && len(blobs) > 0 {
		t.Errorf("path traversal succeeded: got %d blobs", len(blobs))
	}
}

func TestLocalProvider_PathTraversal_ReadBlobMetadata(t *testing.T) {
	p := newTestProvider(t)
	_, err := p.ReadBlobMetadata(context.Background(), "../../../etc/passwd")
	if err == nil {
		t.Error("expected error for path traversal in ReadBlobMetadata, got nil")
	}
}

func TestLocalProvider_PathTraversal_GetBlobURL(t *testing.T) {
	p := newTestProvider(t)
	_, err := p.GetBlobURL(context.Background(), "../../../etc/passwd")
	if err == nil {
		t.Error("expected error for path traversal in GetBlobURL, got nil")
	}
}

func TestLocalProvider_AbsolutePathKey(t *testing.T) {
	p := newTestProvider(t)
	_, err := p.ReadBlobMetadata(context.Background(), "/etc/passwd")
	if err == nil {
		t.Error("expected error for absolute path key, got nil")
	}
}

func TestLocalProvider_ContentType_Default(t *testing.T) {
	dir := t.TempDir()
	p, _ := storage.NewLocalProvider(dir)
	writeFile(t, dir, "file.xyz", "some data")

	meta, err := p.ReadBlobMetadata(context.Background(), "file.xyz")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if meta.ContentType == "" {
		t.Error("expected non-empty ContentType")
	}
}

func TestNewLocalProvider_InvalidPath(t *testing.T) {
	// Passing a null byte in the path should fail on most OSes.
	_, err := storage.NewLocalProvider("/tmp/test\x00dir")
	if err == nil {
		t.Log("null byte in path did not return error (may be OS-dependent)")
	}
}

func TestLocalProvider_ListBlobs_NestedDirectories(t *testing.T) {
	dir := t.TempDir()
	p, _ := storage.NewLocalProvider(dir)
	writeFile(t, dir, "a/b/c/deep.txt", "deep content")
	writeFile(t, dir, "a/b/shallow.txt", "shallow content")
	writeFile(t, dir, "a/top.txt", "top content")

	blobs, err := p.ListBlobs(context.Background(), "a/")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(blobs) != 3 {
		t.Errorf("expected 3 blobs under a/, got %d", len(blobs))
	}
}

func TestLocalProvider_BlobMetadata_ContentType(t *testing.T) {
	dir := t.TempDir()
	p, _ := storage.NewLocalProvider(dir)
	writeFile(t, dir, "data.bin", "binary data")

	meta, err := p.ReadBlobMetadata(context.Background(), "data.bin")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	// The local provider uses a fallback content type.
	if meta.ContentType != "application/octet-stream" {
		t.Errorf("expected application/octet-stream, got %q", meta.ContentType)
	}
}
