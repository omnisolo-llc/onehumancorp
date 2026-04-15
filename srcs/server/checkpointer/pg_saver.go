package checkpointer

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type CheckpointSaver interface {
	GetCheckpoint(ctx context.Context, threadID string) (*CheckpointState, error)
	PutCheckpoint(ctx context.Context, threadID string, checkpoint *CheckpointState) error
	ListCheckpoints(ctx context.Context, threadID string) ([]CheckpointState, error)
}

type CheckpointState struct {
	ThreadID     string                 `json:"thread_id"`
	CheckpointID string                 `json:"checkpoint_id"`
	ParentID     *string                `json:"parent_id"`
	Checkpoint   map[string]interface{} `json:"checkpoint"`
	Metadata     map[string]interface{} `json:"metadata"`
}

type PGSaver struct {
	db db.Provider
}

func NewPGSaver(db db.Provider) *PGSaver {
	return &PGSaver{db: db}
}

func (s *PGSaver) GetCheckpoint(ctx context.Context, threadID string) (*CheckpointState, error) {
	if threadID == "" {
		return nil, errors.New("threadID cannot be empty")
	}

	query := `
		SELECT thread_id, checkpoint_id, parent_id, checkpoint, metadata
		FROM swarm_checkpoints
		WHERE thread_id = $1
		ORDER BY created_at DESC
		LIMIT 1
	`

	var state CheckpointState
	var parentID sql.NullString
	var checkpointBytes, metadataBytes []byte

	err := s.db.QueryRow(ctx, query, threadID).Scan(
		&state.ThreadID,
		&state.CheckpointID,
		&parentID,
		&checkpointBytes,
		&metadataBytes,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, sql.ErrNoRows
		}
		return nil, fmt.Errorf("failed to get checkpoint: %w", err)
	}

	if parentID.Valid {
		state.ParentID = &parentID.String
	}

	if err := json.Unmarshal(checkpointBytes, &state.Checkpoint); err != nil {
		return nil, fmt.Errorf("failed to unmarshal checkpoint: %w", err)
	}

	if len(metadataBytes) > 0 {
		if err := json.Unmarshal(metadataBytes, &state.Metadata); err != nil {
			return nil, fmt.Errorf("failed to unmarshal metadata: %w", err)
		}
	}

	return &state, nil
}

func (s *PGSaver) PutCheckpoint(ctx context.Context, threadID string, checkpoint *CheckpointState) error {
	if threadID == "" || checkpoint.CheckpointID == "" {
		return errors.New("threadID and checkpointID cannot be empty")
	}

	checkpointBytes, err := json.Marshal(checkpoint.Checkpoint)
	if err != nil {
		return fmt.Errorf("failed to marshal checkpoint: %w", err)
	}

	var metadataBytes []byte = []byte("{}")
	if checkpoint.Metadata != nil {
		metadataBytes, err = json.Marshal(checkpoint.Metadata)
		if err != nil {
			return fmt.Errorf("failed to marshal metadata: %w", err)
		}
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
				created_at = CURRENT_TIMESTAMP
		`
	} else {
		query = `
			INSERT INTO swarm_checkpoints (thread_id, checkpoint_id, parent_id, checkpoint, metadata)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (thread_id, checkpoint_id) DO UPDATE SET
				parent_id = EXCLUDED.parent_id,
				checkpoint = EXCLUDED.checkpoint,
				metadata = EXCLUDED.metadata,
				created_at = CURRENT_TIMESTAMP
		`
	}

	var parentID interface{}
	if checkpoint.ParentID != nil {
		parentID = *checkpoint.ParentID
	}

	_, err = s.db.Exec(ctx, query, threadID, checkpoint.CheckpointID, parentID, string(checkpointBytes), string(metadataBytes))
	if err != nil {
		return fmt.Errorf("failed to insert/update checkpoint: %w", err)
	}

	return nil
}

func (s *PGSaver) ListCheckpoints(ctx context.Context, threadID string) ([]CheckpointState, error) {
	if threadID == "" {
		return nil, errors.New("threadID cannot be empty")
	}

	query := `
		SELECT thread_id, checkpoint_id, parent_id, checkpoint, metadata
		FROM swarm_checkpoints
		WHERE thread_id = $1
		ORDER BY created_at ASC
	`

	rows, err := s.db.Query(ctx, query, threadID)
	if err != nil {
		return nil, fmt.Errorf("failed to list checkpoints: %w", err)
	}
	defer rows.Close()

	var checkpoints []CheckpointState
	for rows.Next() {
		var state CheckpointState
		var parentID sql.NullString
		var checkpointBytes, metadataBytes []byte

		if err := rows.Scan(
			&state.ThreadID,
			&state.CheckpointID,
			&parentID,
			&checkpointBytes,
			&metadataBytes,
		); err != nil {
			return nil, fmt.Errorf("failed to scan checkpoint: %w", err)
		}

		if parentID.Valid {
			state.ParentID = &parentID.String
		}

		if err := json.Unmarshal(checkpointBytes, &state.Checkpoint); err != nil {
			return nil, fmt.Errorf("failed to unmarshal checkpoint: %w", err)
		}

		if len(metadataBytes) > 0 {
			if err := json.Unmarshal(metadataBytes, &state.Metadata); err != nil {
				return nil, fmt.Errorf("failed to unmarshal metadata: %w", err)
			}
		}

		checkpoints = append(checkpoints, state)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return checkpoints, nil
}
