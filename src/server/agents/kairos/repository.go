package kairos

import (
	"context"
	"database/sql"
	"fmt"
	"time"
)

// Repository defines the interface for database operations
type Repository interface {
	CreateMission(ctx context.Context, mission *Mission) error
	GetMission(ctx context.Context, id string) (*Mission, error)
	UpdateMissionStatus(ctx context.Context, id string, status MissionStatus, agentID string) error

	AddMissionDependency(ctx context.Context, dependency *MissionDependency) error
	GetMissionDependencies(ctx context.Context, missionID string) ([]MissionDependency, error)

	SaveAutodreamVector(ctx context.Context, vector *AutodreamVector) error
}

type pgRepository struct {
	db *sql.DB
}

// NewRepository creates a new PostgreSQL repository
func NewRepository(db *sql.DB) Repository {
	return &pgRepository{db: db}
}

func (r *pgRepository) CreateMission(ctx context.Context, m *Mission) error {
	query := `
		INSERT INTO ohc_tasks.missions (id, epic_id, title, status, assigned_agent_id, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
	now := time.Now().UTC()
	if m.CreatedAt.IsZero() {
		m.CreatedAt = now
	}
	if m.UpdatedAt.IsZero() {
		m.UpdatedAt = now
	}

	var agentID sql.NullString
	if m.AssignedAgentID != "" {
		agentID = sql.NullString{String: m.AssignedAgentID, Valid: true}
	}

	_, err := r.db.ExecContext(ctx, query,
		m.ID, m.EpicID, m.Title, m.Status, agentID, m.CreatedAt, m.UpdatedAt)
	if err != nil {
		return fmt.Errorf("failed to create mission: %w", err)
	}
	return nil
}

func (r *pgRepository) GetMission(ctx context.Context, id string) (*Mission, error) {
	query := `
		SELECT id, epic_id, title, status, assigned_agent_id, created_at, updated_at
		FROM ohc_tasks.missions
		WHERE id = $1
	`
	var m Mission
	var agentID sql.NullString
	err := r.db.QueryRowContext(ctx, query, id).Scan(
		&m.ID, &m.EpicID, &m.Title, &m.Status, &agentID, &m.CreatedAt, &m.UpdatedAt,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, fmt.Errorf("mission not found")
		}
		return nil, fmt.Errorf("failed to get mission: %w", err)
	}
	if agentID.Valid {
		m.AssignedAgentID = agentID.String
	}
	return &m, nil
}

func (r *pgRepository) UpdateMissionStatus(ctx context.Context, id string, status MissionStatus, agentID string) error {
	query := `
		UPDATE ohc_tasks.missions
		SET status = $1, assigned_agent_id = $2, updated_at = $3
		WHERE id = $4
	`
	now := time.Now().UTC()
	var aID sql.NullString
	if agentID != "" {
		aID = sql.NullString{String: agentID, Valid: true}
	}

	res, err := r.db.ExecContext(ctx, query, status, aID, now, id)
	if err != nil {
		return fmt.Errorf("failed to update mission status: %w", err)
	}
	rows, err := res.RowsAffected()
	if err != nil {
		return err
	}
	if rows == 0 {
		return fmt.Errorf("mission not found")
	}
	return nil
}

func (r *pgRepository) AddMissionDependency(ctx context.Context, dep *MissionDependency) error {
	query := `
		INSERT INTO ohc_tasks.mission_dependencies (id, mission_id, depends_on_mission_id, created_at)
		VALUES ($1, $2, $3, $4)
	`
	now := time.Now().UTC()
	if dep.CreatedAt.IsZero() {
		dep.CreatedAt = now
	}
	_, err := r.db.ExecContext(ctx, query, dep.ID, dep.MissionID, dep.DependsOnMissionID, dep.CreatedAt)
	if err != nil {
		return fmt.Errorf("failed to add mission dependency: %w", err)
	}
	return nil
}

func (r *pgRepository) GetMissionDependencies(ctx context.Context, missionID string) ([]MissionDependency, error) {
	query := `
		SELECT id, mission_id, depends_on_mission_id, created_at
		FROM ohc_tasks.mission_dependencies
		WHERE mission_id = $1
	`
	rows, err := r.db.QueryContext(ctx, query, missionID)
	if err != nil {
		return nil, fmt.Errorf("failed to query dependencies: %w", err)
	}
	defer rows.Close()

	var deps []MissionDependency
	for rows.Next() {
		var d MissionDependency
		if err := rows.Scan(&d.ID, &d.MissionID, &d.DependsOnMissionID, &d.CreatedAt); err != nil {
			return nil, fmt.Errorf("failed to scan dependency: %w", err)
		}
		deps = append(deps, d)
	}
	if err = rows.Err(); err != nil {
		return nil, err
	}
	return deps, nil
}

func (r *pgRepository) SaveAutodreamVector(ctx context.Context, vector *AutodreamVector) error {
	query := `
		INSERT INTO ohc_memory.autodream_vectors (id, mission_id, embedding, created_at)
		VALUES ($1, $2, $3::vector, $4)
	`
	now := time.Now().UTC()
	if vector.CreatedAt.IsZero() {
		vector.CreatedAt = now
	}

	embeddingStr := "["
	for i, v := range vector.Embedding {
		if i > 0 {
			embeddingStr += ","
		}
		embeddingStr += fmt.Sprintf("%f", v)
	}
	embeddingStr += "]"

	_, err := r.db.ExecContext(ctx, query, vector.ID, vector.MissionID, embeddingStr, vector.CreatedAt)
	if err != nil {
		return fmt.Errorf("failed to save autodream vector: %w", err)
	}
	return nil
}
