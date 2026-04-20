package checkpointer

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/db"
	"time"
)

// PgCheckpointSaver implements CheckpointSaver using OHC's db.Provider.
type PgCheckpointSaver struct {
	provider db.Provider
}

// NewPgCheckpointSaver creates a new PgCheckpointSaver.
func NewPgCheckpointSaver(provider db.Provider) *PgCheckpointSaver {
	return &PgCheckpointSaver{provider: provider}
}

// GetCheckpoint retrieves a specific checkpoint.
func (s *PgCheckpointSaver) GetCheckpoint(ctx context.Context, threadID string, checkpointID string) (*Checkpoint, error) {
	query := `
		SELECT thread_id, checkpoint_id, parent_id, checkpoint, metadata, created_at
		FROM swarm_checkpoints
		WHERE thread_id = $1 AND checkpoint_id = $2
	`
	row := s.provider.QueryRow(ctx, query, threadID, checkpointID)

	var cp Checkpoint
	var checkpointRaw, metadataRaw []byte
	var createdAt interface{}

	err := row.Scan(&cp.ThreadID, &cp.CheckpointID, &cp.ParentID, &checkpointRaw, &metadataRaw, &createdAt)
	if err != nil {
		return nil, err
	}

	cp.CreatedAt, err = parseTime(createdAt)
	if err != nil {
		return nil, fmt.Errorf("failed to parse created_at: %w", err)
	}

	if err := json.Unmarshal(checkpointRaw, &cp.Data); err != nil {
		return nil, fmt.Errorf("failed to unmarshal checkpoint data: %w", err)
	}

	if err := json.Unmarshal(metadataRaw, &cp.Metadata); err != nil {
		return nil, fmt.Errorf("failed to unmarshal metadata: %w", err)
	}

	return &cp, nil
}

// PutCheckpoint saves a checkpoint.
func (s *PgCheckpointSaver) PutCheckpoint(ctx context.Context, cp *Checkpoint) error {
	checkpointRaw, err := json.Marshal(cp.Data)
	if err != nil {
		return fmt.Errorf("failed to marshal checkpoint data: %w", err)
	}

	metadataRaw, err := json.Marshal(cp.Metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	query := `
		INSERT INTO swarm_checkpoints (thread_id, checkpoint_id, parent_id, checkpoint, metadata, created_at)
		VALUES ($1, $2, $3, $4, $5, $6)
		ON CONFLICT (thread_id, checkpoint_id) DO UPDATE SET
			parent_id = EXCLUDED.parent_id,
			checkpoint = EXCLUDED.checkpoint,
			metadata = EXCLUDED.metadata,
			created_at = EXCLUDED.created_at
	`

	_, err = s.provider.Exec(ctx, query, cp.ThreadID, cp.CheckpointID, cp.ParentID, checkpointRaw, metadataRaw, cp.CreatedAt)
	if err != nil {
		return fmt.Errorf("failed to put checkpoint: %w", err)
	}

	return nil
}

// ListCheckpoints returns all checkpoints for a thread.
func (s *PgCheckpointSaver) ListCheckpoints(ctx context.Context, threadID string) ([]Checkpoint, error) {
	query := `
		SELECT thread_id, checkpoint_id, parent_id, checkpoint, metadata, created_at
		FROM swarm_checkpoints
		WHERE thread_id = $1
		ORDER BY created_at DESC
	`
	rows, err := s.provider.Query(ctx, query, threadID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var checkpoints []Checkpoint
	for rows.Next() {
		var cp Checkpoint
		var checkpointRaw, metadataRaw []byte
		var createdAt interface{}
		if err := rows.Scan(&cp.ThreadID, &cp.CheckpointID, &cp.ParentID, &checkpointRaw, &metadataRaw, &createdAt); err != nil {
			return nil, err
		}

		cp.CreatedAt, err = parseTime(createdAt)
		if err != nil {
			return nil, fmt.Errorf("failed to parse created_at: %w", err)
		}

		if err := json.Unmarshal(checkpointRaw, &cp.Data); err != nil {
			return nil, fmt.Errorf("failed to unmarshal checkpoint data: %w", err)
		}

		if err := json.Unmarshal(metadataRaw, &cp.Metadata); err != nil {
			return nil, fmt.Errorf("failed to unmarshal metadata: %w", err)
		}

		checkpoints = append(checkpoints, cp)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}

	return checkpoints, nil
}

func parseTime(raw interface{}) (time.Time, error) {
	switch v := raw.(type) {
	case time.Time:
		return v, nil
	case string:
		// SQLite often returns times as strings
		t, err := time.Parse(time.RFC3339, v)
		if err == nil {
			return t, nil
		}
		t, err = time.Parse("2006-01-02 15:04:05", v)
		if err == nil {
			return t, nil
		}
		return time.Parse("2006-01-02 15:04:05 -0700 MST", v)
	case []byte:
		return parseTime(string(v))
	default:
		return time.Time{}, fmt.Errorf("unsupported time type: %T", v)
	}
}
