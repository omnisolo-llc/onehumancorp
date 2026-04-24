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

func decompressData(data []byte) ([]byte, error) {
	isQuoted := len(data) >= 2 && data[0] == '"' && data[len(data)-1] == '"'

	var decodeInput []byte
	if isQuoted {
		decodeInput = data[1 : len(data)-1]
	} else {
		decodeInput = data
	}

	decoded := make([]byte, base64.StdEncoding.DecodedLen(len(decodeInput)))
	n, err := base64.StdEncoding.Decode(decoded, decodeInput)
	if err != nil {
		return data, nil
	}

	gzr, err := gzip.NewReader(bytes.NewReader(decoded[:n]))
	if err != nil {
		return data, nil
	}
	defer gzr.Close()

	decompressed, err := io.ReadAll(gzr)
	if err != nil {
		return data, nil
	}

	return decompressed, nil
}

func compressData(data []byte) ([]byte, error) {
	var buf bytes.Buffer
	gzw := gzip.NewWriter(&buf)
	if _, err := gzw.Write(data); err != nil {
		return nil, err
	}
	if err := gzw.Close(); err != nil {
		return nil, err
	}

	encodedLen := base64.StdEncoding.EncodedLen(buf.Len())
	result := make([]byte, encodedLen+2)
	result[0] = '"'
	base64.StdEncoding.Encode(result[1:encodedLen+1], buf.Bytes())
	result[encodedLen+1] = '"'
	return result, nil
}
