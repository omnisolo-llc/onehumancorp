package repositories

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/db/models"
)

type MeshRepository interface {
	CreateMission(ctx context.Context, mission *models.Mission) error
	UpdateMissionStatus(ctx context.Context, id, status string) error
	GetMissionDependencies(ctx context.Context, missionID string) ([]string, error)
	InsertAutodreamVector(ctx context.Context, vector *models.AutodreamVector) error
}

type meshRepositoryImpl struct {
	dbProvider db.Provider
}

func NewMeshRepository(dbProvider db.Provider) MeshRepository {
	return &meshRepositoryImpl{
		dbProvider: dbProvider,
	}
}

func (r *meshRepositoryImpl) CreateMission(ctx context.Context, mission *models.Mission) error {
	q := `INSERT INTO ohc_tasks.missions (id, epic_id, title, status, assigned_agent_id) VALUES ($1, $2, $3, $4, $5)`

	// Handle sqlite fallback for schemas
	if r.dbProvider.IsSQLite() {
		q = `INSERT INTO missions (id, epic_id, title, status, assigned_agent_id) VALUES ($1, $2, $3, $4, $5)`
	}

	_, err := r.dbProvider.Exec(ctx, q, mission.ID, mission.EpicID, mission.Title, mission.Status, mission.AssignedAgentID)
	if err != nil {
		return fmt.Errorf("failed to insert mission: %w", err)
	}

	return nil
}

func (r *meshRepositoryImpl) UpdateMissionStatus(ctx context.Context, id, status string) error {
	q := `UPDATE ohc_tasks.missions SET status = $1 WHERE id = $2`
	if r.dbProvider.IsSQLite() {
		q = `UPDATE missions SET status = $1 WHERE id = $2`
	}

	_, err := r.dbProvider.Exec(ctx, q, status, id)
	if err != nil {
		return fmt.Errorf("failed to update mission status: %w", err)
	}

	return nil
}

func (r *meshRepositoryImpl) GetMissionDependencies(ctx context.Context, missionID string) ([]string, error) {
	q := `SELECT depends_on_id FROM ohc_tasks.mission_dependencies WHERE mission_id = $1`
	if r.dbProvider.IsSQLite() {
		q = `SELECT depends_on_id FROM mission_dependencies WHERE mission_id = $1`
	}

	rows, err := r.dbProvider.Query(ctx, q, missionID)
	if err != nil {
		return nil, fmt.Errorf("failed to query mission dependencies: %w", err)
	}
	defer rows.Close()

	var deps []string
	for rows.Next() {
		var depID string
		if err := rows.Scan(&depID); err != nil {
			return nil, fmt.Errorf("failed to scan dep id: %w", err)
		}
		deps = append(deps, depID)
	}
	if err = rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return deps, nil
}

func (r *meshRepositoryImpl) InsertAutodreamVector(ctx context.Context, vector *models.AutodreamVector) error {
	q := `INSERT INTO ohc_memory.autodream_vectors (id, task_id, content, embedding, metadata, created_at) VALUES ($1, $2, $3, $4, $5, $6)`
	if r.dbProvider.IsSQLite() {
		q = `INSERT INTO autodream_vectors (id, task_id, content, embedding, metadata, created_at) VALUES ($1, $2, $3, $4, $5, $6)`
	}

	embBytes, err := json.Marshal(vector.Embedding)
	if err != nil {
		return fmt.Errorf("failed to marshal embedding: %w", err)
	}

	metaBytes, err := json.Marshal(vector.Metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	if vector.CreatedAt.IsZero() {
		vector.CreatedAt = time.Now()
	}

	_, err = r.dbProvider.Exec(ctx, q, vector.ID, vector.TaskID, vector.Content, string(embBytes), string(metaBytes), vector.CreatedAt)
	if err != nil {
		return fmt.Errorf("failed to insert autodream vector: %w", err)
	}

	return nil
}
