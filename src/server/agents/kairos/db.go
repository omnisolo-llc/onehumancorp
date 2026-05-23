package kairos

import (
	"context"
	"database/sql"
	"time"

	"github.com/google/uuid"
)

type Mission struct {
	ID              uuid.UUID `json:"id" db:"id"`
	EpicID          uuid.UUID `json:"epic_id" db:"epic_id"`
	Title           string    `json:"title" db:"title"`
	Status          string    `json:"status" db:"status"`
	AssignedAgentID *string   `json:"assigned_agent_id" db:"assigned_agent_id"`
}

type MissionDependency struct {
	ID                 uuid.UUID `json:"id" db:"id"`
	MissionID          uuid.UUID `json:"mission_id" db:"mission_id"`
	DependsOnMissionID uuid.UUID `json:"depends_on_mission_id" db:"depends_on_mission_id"`
}

type AutodreamVector struct {
	ID         uuid.UUID `json:"id" db:"id"`
	MissionID  uuid.UUID `json:"mission_id" db:"mission_id"`
	VectorData string    `json:"vector_data" db:"vector_data"`
	CreatedAt  time.Time `json:"created_at" db:"created_at"`
}

type KairosRepository interface {
	CreateMission(ctx context.Context, mission *Mission) error
	GetMission(ctx context.Context, id uuid.UUID) (*Mission, error)
	CreateMissionDependency(ctx context.Context, dep *MissionDependency) error
	CreateAutodreamVector(ctx context.Context, vector *AutodreamVector) error
}

type sqlKairosRepository struct {
	db *sql.DB
}

func NewKairosRepository(db *sql.DB) KairosRepository {
	return &sqlKairosRepository{db: db}
}

func (r *sqlKairosRepository) CreateMission(ctx context.Context, mission *Mission) error {
	query := `
		INSERT INTO ohc_tasks.missions (id, epic_id, title, status, assigned_agent_id)
		VALUES ($1, $2, $3, $4, $5)
	`
	_, err := r.db.ExecContext(ctx, query, mission.ID, mission.EpicID, mission.Title, mission.Status, mission.AssignedAgentID)
	return err
}

func (r *sqlKairosRepository) GetMission(ctx context.Context, id uuid.UUID) (*Mission, error) {
	query := `
		SELECT id, epic_id, title, status, assigned_agent_id
		FROM ohc_tasks.missions
		WHERE id = $1
	`
	row := r.db.QueryRowContext(ctx, query, id)
	var m Mission
	err := row.Scan(&m.ID, &m.EpicID, &m.Title, &m.Status, &m.AssignedAgentID)
	if err != nil {
		return nil, err
	}
	return &m, nil
}

func (r *sqlKairosRepository) CreateMissionDependency(ctx context.Context, dep *MissionDependency) error {
	query := `
		INSERT INTO ohc_tasks.mission_dependencies (id, mission_id, depends_on_mission_id)
		VALUES ($1, $2, $3)
	`
	_, err := r.db.ExecContext(ctx, query, dep.ID, dep.MissionID, dep.DependsOnMissionID)
	return err
}

func (r *sqlKairosRepository) CreateAutodreamVector(ctx context.Context, vector *AutodreamVector) error {
	query := `
		INSERT INTO ohc_memory.autodream_vectors (id, mission_id, vector_data, created_at)
		VALUES ($1, $2, $3::vector, $4)
	`
	var createdAt time.Time
	if vector.CreatedAt.IsZero() {
		createdAt = time.Now()
	} else {
		createdAt = vector.CreatedAt
	}

	_, err := r.db.ExecContext(ctx, query, vector.ID, vector.MissionID, vector.VectorData, createdAt)
	return err
}
