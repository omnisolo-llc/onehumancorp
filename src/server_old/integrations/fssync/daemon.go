package fssync

import (
	"context"
	"io"
	"os"
)

// FileChunk represents a chunk of a file
type FileChunk struct {
	Path        string
	ChunkIndex  int
	TotalChunks int
	Data        []byte
}

// Uploader interface for uploading file chunks to the cloud
type Uploader interface {
	Upload(ctx context.Context, chunk FileChunk) error
}

// MockUploader is a mock implementation of Uploader for testing
type MockUploader struct {
	UploadedChunks []FileChunk
}

// NewMockUploader creates a new MockUploader
func NewMockUploader() *MockUploader {
	return &MockUploader{
		UploadedChunks: make([]FileChunk, 0),
	}
}

// Upload mocks the upload process
func (m *MockUploader) Upload(ctx context.Context, chunk FileChunk) error {
	m.UploadedChunks = append(m.UploadedChunks, chunk)
	return nil
}

// SyncDaemon consumes events, chunks files, and uploads them
type SyncDaemon struct {
	watcher   Watcher
	uploader  Uploader
	chunkSize int
}

// NewSyncDaemon creates a new SyncDaemon
func NewSyncDaemon(watcher Watcher, uploader Uploader, chunkSize int) *SyncDaemon {
	if chunkSize <= 0 {
		chunkSize = 1024 * 1024 // 1MB default
	}
	return &SyncDaemon{
		watcher:   watcher,
		uploader:  uploader,
		chunkSize: chunkSize,
	}
}

// Start begins listening to the watcher and processing events
func (d *SyncDaemon) Start(ctx context.Context) error {
	events, err := d.watcher.Watch(ctx)
	if err != nil {
		return err
	}

	for {
		select {
		case <-ctx.Done():
			return nil
		case event, ok := <-events:
			if !ok {
				return nil
			}
			if event.Operation == "WRITE" {
				if err := d.processFile(ctx, event.Path); err != nil {
					// Log error, but continue running
					continue
				}
			}
		}
	}
}

// processFile reads a file, chunks it, and uploads the chunks
func (d *SyncDaemon) processFile(ctx context.Context, path string) error {
	file, err := os.Open(path)
	if err != nil {
		return err
	}
	defer file.Close()

	stat, err := file.Stat()
	if err != nil {
		return err
	}

	fileSize := stat.Size()
	totalChunks := int((fileSize + int64(d.chunkSize) - 1) / int64(d.chunkSize))
	if totalChunks == 0 {
		totalChunks = 1 // Even empty files get 1 chunk
	}

	chunkIndex := 0
	buf := make([]byte, d.chunkSize)

	for {
		n, err := file.Read(buf)
		if err != nil && err != io.EOF {
			return err
		}
		if n == 0 && err == io.EOF {
			// Empty file case
			if fileSize == 0 {
				chunk := FileChunk{
					Path:        path,
					ChunkIndex:  0,
					TotalChunks: 1,
					Data:        []byte{},
				}
				if err := d.uploader.Upload(ctx, chunk); err != nil {
					return err
				}
			}
			break
		}

		chunkData := make([]byte, n)
		copy(chunkData, buf[:n])

		chunk := FileChunk{
			Path:        path,
			ChunkIndex:  chunkIndex,
			TotalChunks: totalChunks,
			Data:        chunkData,
		}

		if err := d.uploader.Upload(ctx, chunk); err != nil {
			return err
		}

		chunkIndex++
		if err == io.EOF {
			break
		}
	}

	return nil
}
