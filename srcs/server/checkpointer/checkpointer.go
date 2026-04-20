package checkpointer

import (
	"bytes"
	"compress/gzip"
	"context"
	"encoding/base64"
	"io"
	"time"
)

// Checkpoint represents a single state snapshot for a given LangGraph thread.
type Checkpoint struct {
	ThreadID     string                 `json:"thread_id"`
	CheckpointID string                 `json:"checkpoint_id"`
	ParentID     *string                `json:"parent_id"`
	Data         map[string]interface{} `json:"checkpoint"`
	Metadata     map[string]interface{} `json:"metadata"`
	CreatedAt    time.Time              `json:"created_at"`
}

// CheckpointSaver interface defines the required methods for saving and loading agent states.
type CheckpointSaver interface {
	GetCheckpoint(ctx context.Context, threadID string, checkpointID string) (*Checkpoint, error)
	PutCheckpoint(ctx context.Context, checkpoint *Checkpoint) error
	ListCheckpoints(ctx context.Context, threadID string) ([]Checkpoint, error)
}

// compressData compresses the given byte slice using gzip and encodes it to base64 string.
func compressData(data []byte) ([]byte, error) {
	var b bytes.Buffer
	w := gzip.NewWriter(&b)
	_, err := w.Write(data)
	if err != nil {
		return nil, err
	}
	err = w.Close()
	if err != nil {
		return nil, err
	}

	encoded := base64.StdEncoding.EncodeToString(b.Bytes())
	return []byte(encoded), nil
}

// decompressData decodes the base64 string and decompresses the underlying byte slice using gzip.
// If the data is not valid base64 or not a valid gzip stream, it assumes it's uncompressed plain text (backward compatibility).
func decompressData(data []byte) ([]byte, error) {
	decodedBytes := make([]byte, base64.StdEncoding.DecodedLen(len(data)))
	n, err := base64.StdEncoding.Decode(decodedBytes, data)
	if err != nil {
		return data, nil
	}
	decodedBytes = decodedBytes[:n]

	if len(decodedBytes) < 2 || decodedBytes[0] != 0x1f || decodedBytes[1] != 0x8b {
		return data, nil
	}

	r, err := gzip.NewReader(bytes.NewReader(decodedBytes))
	if err != nil {
		return data, nil
	}
	defer r.Close()

	decompressed, err := io.ReadAll(r)
	if err != nil {
		return data, nil
	}
	return decompressed, nil
}
