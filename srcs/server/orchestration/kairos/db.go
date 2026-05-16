package kairos

import (
	"context"
	"database/sql"
	"fmt"
	"strings"
)

type Mission struct {
	ID              string
	EpicID          string
	Title           string
	Status          string
	AssignedAgentID string
}

type MissionDependency struct {
	ID                 string
	MissionID          string
	DependsOnMissionID string
}

type AutodreamVector struct {
	ID        string
	Embedding []float32
	Payload   string
}

type Repository interface {
	CreateMission(ctx context.Context, m *Mission) error
	GetMission(ctx context.Context, id string) (*Mission, error)
	CreateDependency(ctx context.Context, d *MissionDependency) error
	CreateAutodreamVector(ctx context.Context, v *AutodreamVector) error
}

type PostgresRepository struct {
	db *sql.DB
}

func NewPostgresRepository(db *sql.DB) *PostgresRepository {
	return &PostgresRepository{db: db}
}

func (r *PostgresRepository) CreateMission(ctx context.Context, m *Mission) error {
	query := "INSERT INTO ohc_tasks.missions (id, epic_id, title, status, assigned_agent_id) VALUES ($1, $2, $3, $4, $5)"
	_, err := r.db.ExecContext(ctx, query, m.ID, m.EpicID, m.Title, m.Status, m.AssignedAgentID)
	return err
}

func (r *PostgresRepository) GetMission(ctx context.Context, id string) (*Mission, error) {
	query := "SELECT id, epic_id, title, status, assigned_agent_id FROM ohc_tasks.missions WHERE id = $1"
	row := r.db.QueryRowContext(ctx, query, id)
	var m Mission
	err := row.Scan(&m.ID, &m.EpicID, &m.Title, &m.Status, &m.AssignedAgentID)
	if err != nil {
		return nil, err
	}
	return &m, nil
}

func (r *PostgresRepository) CreateDependency(ctx context.Context, d *MissionDependency) error {
	query := "INSERT INTO ohc_tasks.mission_dependencies (id, mission_id, depends_on_mission_id) VALUES ($1, $2, $3)"
	_, err := r.db.ExecContext(ctx, query, d.ID, d.MissionID, d.DependsOnMissionID)
	return err
}

func (r *PostgresRepository) CreateAutodreamVector(ctx context.Context, v *AutodreamVector) error {
	var strValues []string
	for _, val := range v.Embedding {
		strValues = append(strValues, fmt.Sprintf("%f", val))
	}
	vectorStr := fmt.Sprintf("[%s]", strings.Join(strValues, ","))

	query := "INSERT INTO ohc_memory.autodream_vectors (id, embedding, payload) VALUES ($1, $2, $3)"
	_, err := r.db.ExecContext(ctx, query, v.ID, vectorStr, v.Payload)
	return err
}
