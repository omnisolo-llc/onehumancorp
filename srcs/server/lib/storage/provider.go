package storage

import "context"

type BlobProvider interface {
	WriteBlob(ctx context.Context, path string, data []byte) error
	ReadBlob(ctx context.Context, path string) ([]byte, error)
}

func ConvertToWebP(data []byte) []byte {
	// Mock conversion logic: in reality, this would use a library to transcode the image buffer to WebP.
	// For testing and simulation purposes, we just return the original data and log a WebP marker.
	return append(data, []byte("_webp_converted")...)
}
