package fssync

import (
	"context"
	"fmt"
	"log"
	"sync"
	"time"
)

// FileEvent represents a file system modification event
type FileEvent struct {
	FilePath  string
	Operation string // "WRITE", "DELETE", etc.
	Timestamp time.Time
}

// Watcher interface defines listening to file system changes in a given directory
type Watcher interface {
	Watch(ctx context.Context, dirPath string) (<-chan FileEvent, error)
}

// Chunker interface defines breaking files into manageable chunks
type Chunker interface {
	Chunk(filePath string) ([][]byte, error)
}

// Uploader interface defines pushing chunks to a cloud URL
type Uploader interface {
	Upload(ctx context.Context, chunks [][]byte, metadata map[string]string) error
}

// SyncDaemon consumes events from the Watcher, chunks files, and queues them for upload.
type SyncDaemon struct {
	watcher  Watcher
	chunker  Chunker
	uploader Uploader
	dirPath  string
	queue    chan FileEvent
	wg       sync.WaitGroup
}

// NewSyncDaemon creates a new SyncDaemon
func NewSyncDaemon(w Watcher, c Chunker, u Uploader, dirPath string) *SyncDaemon {
	return &SyncDaemon{
		watcher:  w,
		chunker:  c,
		uploader: u,
		dirPath:  dirPath,
		queue:    make(chan FileEvent, 100),
	}
}

// Start begins consuming events from the Watcher and processing them
func (d *SyncDaemon) Start(ctx context.Context) error {
	events, err := d.watcher.Watch(ctx, d.dirPath)
	if err != nil {
		return fmt.Errorf("failed to start watcher: %w", err)
	}

	d.wg.Add(1)
	go func() {
		defer d.wg.Done()
		for {
			select {
			case <-ctx.Done():
				return
			case event, ok := <-events:
				if !ok {
					return
				}
				d.queue <- event
			}
		}
	}()

	d.wg.Add(1)
	go d.processQueue(ctx)

	return nil
}

func (d *SyncDaemon) processQueue(ctx context.Context) {
	defer d.wg.Done()
	for {
		select {
		case <-ctx.Done():
			return
		case event := <-d.queue:
			if event.Operation == "WRITE" {
				chunks, err := d.chunker.Chunk(event.FilePath)
				if err != nil {
					log.Printf("failed to chunk file %s: %v", event.FilePath, err)
					continue
				}

				metadata := map[string]string{
					"filepath":  event.FilePath,
					"operation": event.Operation,
					"timestamp": event.Timestamp.Format(time.RFC3339),
				}

				err = d.uploader.Upload(ctx, chunks, metadata)
				if err != nil {
					log.Printf("failed to upload chunks for %s: %v", event.FilePath, err)
				}
			}
		}
	}
}

// Stop waits for pending operations to finish
func (d *SyncDaemon) Stop() {
	d.wg.Wait()
}
