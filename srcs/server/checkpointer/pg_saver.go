package checkpointer

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type PgCheckpointSaver struct {
	db db.Provider
}

func NewPgCheckpointSaver(provider db.Provider) *PgCheckpointSaver {
	return &PgCheckpointSaver{db: provider}
}

func (s *PgCheckpointSaver) PutCheckpoint(ctx context.Context, threadID string, checkpoint *Checkpoint) error {
	checkpointJSON, err := json.Marshal(checkpoint.Checkpoint)
	if err != nil {
		return fmt.Errorf("failed to marshal checkpoint: %w", err)
	}

	metadataJSON, err := json.Marshal(checkpoint.Metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	var query string
	if s.db.IsSQLite() {
		query = `
			INSERT INTO swarm_checkpoints (thread_id, checkpoint_id, parent_id, checkpoint, metadata)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (thread_id, checkpoint_id) DO UPDATE SET
				parent_id = excluded.parent_id,
				checkpoint = excluded.checkpoint,
				metadata = excluded.metadata,
				created_at = CURRENT_TIMESTAMP;
		`
	} else {
		query = `
			INSERT INTO swarm_checkpoints (thread_id, checkpoint_id, parent_id, checkpoint, metadata)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (thread_id, checkpoint_id) DO UPDATE SET
				parent_id = EXCLUDED.parent_id,
				checkpoint = EXCLUDED.checkpoint,
				metadata = EXCLUDED.metadata,
				created_at = CURRENT_TIMESTAMP;
		`
	}

	_, err = s.db.Exec(ctx, query, threadID, checkpoint.CheckpointID, checkpoint.ParentID, string(checkpointJSON), string(metadataJSON))
	if err != nil {
		return fmt.Errorf("failed to put checkpoint: %w", err)
	}

	return nil
}

func (s *PgCheckpointSaver) GetCheckpoint(ctx context.Context, threadID string) (*Checkpoint, error) {
	query := `
		SELECT thread_id, checkpoint_id, parent_id, checkpoint, metadata
		FROM swarm_checkpoints
		WHERE thread_id = $1
		ORDER BY created_at DESC
		LIMIT 1
	`
	row := s.db.QueryRow(ctx, query, threadID)

	var cp Checkpoint
	var checkpointStr, metadataStr string
	if err := row.Scan(&cp.ThreadID, &cp.CheckpointID, &cp.ParentID, &checkpointStr, &metadataStr); err != nil {
		return nil, err
	}

	if err := json.Unmarshal([]byte(checkpointStr), &cp.Checkpoint); err != nil {
		return nil, fmt.Errorf("failed to unmarshal checkpoint: %w", err)
	}

	if err := json.Unmarshal([]byte(metadataStr), &cp.Metadata); err != nil {
		return nil, fmt.Errorf("failed to unmarshal metadata: %w", err)
	}

	return &cp, nil
}

func (s *PgCheckpointSaver) ListCheckpoints(ctx context.Context, threadID string) ([]Checkpoint, error) {
	query := `
		SELECT thread_id, checkpoint_id, parent_id, checkpoint, metadata
		FROM swarm_checkpoints
		WHERE thread_id = $1
		ORDER BY created_at DESC
	`
	rows, err := s.db.Query(ctx, query, threadID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var checkpoints []Checkpoint
	for rows.Next() {
		var cp Checkpoint
		var checkpointStr, metadataStr string
		if err := rows.Scan(&cp.ThreadID, &cp.CheckpointID, &cp.ParentID, &checkpointStr, &metadataStr); err != nil {
			return nil, err
		}

		if err := json.Unmarshal([]byte(checkpointStr), &cp.Checkpoint); err != nil {
			return nil, fmt.Errorf("failed to unmarshal checkpoint: %w", err)
		}

		if err := json.Unmarshal([]byte(metadataStr), &cp.Metadata); err != nil {
			return nil, fmt.Errorf("failed to unmarshal metadata: %w", err)
		}

		checkpoints = append(checkpoints, cp)
	}

	return checkpoints, nil
}
