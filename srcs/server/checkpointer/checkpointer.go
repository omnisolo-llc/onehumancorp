package checkpointer

import (
	"bytes"
	"compress/gzip"
	"context"
	"database/sql"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"math/rand"
	"time"
)

// compressData compresses the given byte slice using gzip and encodes it to base64 string.
func compressData(data []byte) (string, error) {
	var b bytes.Buffer
	w := gzip.NewWriter(&b)
	_, err := w.Write(data)
	if err != nil {
		return "", err
	}
	err = w.Close()
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(b.Bytes()), nil
}

// decompressData decodes the base64 string and decompresses the underlying byte slice using gzip.
func decompressData(base64Str string) ([]byte, error) {
	decodedBytes, err := base64.StdEncoding.DecodeString(base64Str)
	if err != nil {
		return nil, err
	}
	r, err := gzip.NewReader(bytes.NewReader(decodedBytes))
	if err != nil {
		return nil, err
	}
	defer r.Close()
	return io.ReadAll(r)
}

// Checkpoint represents a single state snapshot for a given LangGraph thread.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Checkpoint struct {
	ThreadID string                 `json:"thread_id"`
	State    map[string]interface{} `json:"state"`
}

// LangGraphCheckpointer interface defines the required methods for saving and loading.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type LangGraphCheckpointer interface {
	SaveCheckpoint(ctx context.Context, threadID string, state map[string]interface{}) error
	LoadCheckpoint(ctx context.Context, threadID string) (*Checkpoint, error)
}

// PGCheckpointer implements LangGraphCheckpointer using a database backend.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type PGCheckpointer struct {
	db *sql.DB
}

// NewPGCheckpointer creates a new instance of PGCheckpointer.
// Accepts parameters: db *sql.DB (No Constraints).
// Returns *PGCheckpointer.
// Produces no errors.
// Has no side effects.
func NewPGCheckpointer(db *sql.DB) *PGCheckpointer {
	return &PGCheckpointer{db: db}
}

// EnsureTableExists creates the checkpoints table if it does not already exist.
// Accepts parameters: p *PGCheckpointer (No Constraints).
// Returns EnsureTableExists(ctx context.Context) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (p *PGCheckpointer) EnsureTableExists(ctx context.Context) error {
	query := `
	CREATE TABLE IF NOT EXISTS checkpoints (
		thread_id VARCHAR(255) PRIMARY KEY,
		state JSONB NOT NULL
	);`
	return p.withRetry(func() error {
		_, err := p.db.ExecContext(ctx, query)
		return err
	})
}

// SaveCheckpoint serializes and persists the given state for the specified threadID.
// Accepts parameters: p *PGCheckpointer (No Constraints).
// Returns SaveCheckpoint(ctx context.Context, threadID string, state map[string]interface{}) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (p *PGCheckpointer) SaveCheckpoint(ctx context.Context, threadID string, state map[string]interface{}) error {
	if threadID == "" {
		return errors.New("threadID cannot be empty")
	}

	stateBytes, err := json.Marshal(state)
	if err != nil {
		return fmt.Errorf("failed to marshal state: %w", err)
	}

	compressedBase64, err := compressData(stateBytes)
	if err != nil {
		return fmt.Errorf("failed to compress state: %w", err)
	}

	// Wrap the base64 string in a valid JSON object.
	wrapper := map[string]string{
		"_compressed_base64": compressedBase64,
	}
	wrapperBytes, err := json.Marshal(wrapper)
	if err != nil {
		return fmt.Errorf("failed to marshal compressed wrapper: %w", err)
	}

	// Use an upsert strategy. The specific syntax here works for both SQLite (for tests) and PostgreSQL
	// Postgres uses ON CONFLICT. SQLite uses ON CONFLICT (from 3.24.0+).
	query := `
	INSERT INTO checkpoints (thread_id, state)
	VALUES ($1, $2)
	ON CONFLICT (thread_id) DO UPDATE SET state = excluded.state;
	`
	return p.withRetry(func() error {
		_, err := p.db.ExecContext(ctx, query, threadID, wrapperBytes)
		return err
	})
}

// LoadCheckpoint retrieves and deserializes the state for a given threadID.
// Accepts parameters: p *PGCheckpointer (No Constraints).
// Returns LoadCheckpoint(ctx context.Context, threadID string) (*Checkpoint, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (p *PGCheckpointer) LoadCheckpoint(ctx context.Context, threadID string) (*Checkpoint, error) {
	if threadID == "" {
		return nil, errors.New("threadID cannot be empty")
	}

	query := `SELECT thread_id, state FROM checkpoints WHERE thread_id = $1;`

	var id string
	var stateBytes []byte

	err := p.withRetry(func() error {
		return p.db.QueryRowContext(ctx, query, threadID).Scan(&id, &stateBytes)
	})
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, sql.ErrNoRows
		}
		return nil, fmt.Errorf("failed to query checkpoint: %w", err)
	}

	// Check if this is a compressed wrapper or raw state
	var wrapper struct {
		CompressedBase64 string `json:"_compressed_base64"`
	}
	if err := json.Unmarshal(stateBytes, &wrapper); err == nil {
		if wrapper.CompressedBase64 != "" {
			decompressedBytes, err := decompressData(wrapper.CompressedBase64)
			if err != nil {
				return nil, fmt.Errorf("failed to decompress state: %w", err)
			}
			stateBytes = decompressedBytes
		}
	}

	// Strictly decode JSON as required by memory.
	dec := json.NewDecoder(bytes.NewReader(stateBytes))
	dec.DisallowUnknownFields()

	var state map[string]interface{}
	if err := dec.Decode(&state); err != nil {
		return nil, fmt.Errorf("failed to decode state: %w", err)
	}

	return &Checkpoint{
		ThreadID: id,
		State:    state,
	}, nil
}

// withRetry wraps a database operation in an exponential backoff retry loop
// to handle transient errors like SQLite locking ("database is locked").
func (p *PGCheckpointer) withRetry(operation func() error) error {
	maxRetries := 5
	baseDelay := 10 * time.Millisecond
	maxDelay := 500 * time.Millisecond

	for i := 0; i < maxRetries; i++ {
		err := operation()
		if err == nil {
			return nil
		}

		// Basic check for SQLite busy/locked errors, or transient connection issues
		errMsg := err.Error()
		if errMsg == "database is locked" || errMsg == "database table is locked" {
			// Calculate exponential backoff with jitter
			delay := baseDelay * (1 << i)
			if delay > maxDelay {
				delay = maxDelay
			}
			// Add up to 20% jitter
			jitter := time.Duration(rand.Int63n(int64(delay) / 5))
			time.Sleep(delay + jitter)
			continue
		}

		// If it's not a known transient error, return immediately
		return err
	}

	return fmt.Errorf("operation failed after %d retries", maxRetries)
}
