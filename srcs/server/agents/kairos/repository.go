package kairos

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type MissionRepository interface {
	CreateMission(ctx context.Context, mission *Mission) error
	GetMission(ctx context.Context, id uuid.UUID) (*Mission, error)
	UpdateMissionStatus(ctx context.Context, id uuid.UUID, status string) error
	AddDependency(ctx context.Context, taskID, dependsOnTaskID uuid.UUID) error
	GetDependencies(ctx context.Context, taskID uuid.UUID) ([]uuid.UUID, error)
	CreateAutodreamVector(ctx context.Context, vector *AutodreamVector) error
}

type missionRepositoryImpl struct {
	dbProvider db.Provider
}

func NewMissionRepository(dbProvider db.Provider) MissionRepository {
	return &missionRepositoryImpl{
		dbProvider: dbProvider,
	}
}

func (r *missionRepositoryImpl) getTableName(tableName string) string {
	if r.dbProvider.IsSQLite() {
		// Convert schema.table to schema_table for SQLite compatibility
		if tableName == "ohc_tasks.missions" {
			return "ohc_tasks_missions"
		}
		if tableName == "ohc_tasks.mission_dependencies" {
			return "ohc_tasks_mission_dependencies"
		}
		if tableName == "ohc_memory.autodream_vectors" {
			return "ohc_memory_autodream_vectors"
		}
	}
	return tableName
}

func (r *missionRepositoryImpl) CreateMission(ctx context.Context, mission *Mission) error {
	tableName := r.getTableName("ohc_tasks.missions")
	q := fmt.Sprintf(`INSERT INTO %s (id, epic_id, title, status, assigned_agent_id, created_at, updated_at)
		  VALUES ($1, $2, $3, $4, $5, $6, $7)`, tableName)

	now := time.Now().UTC()
	if mission.ID == uuid.Nil {
		mission.ID = uuid.New()
	}
	if mission.CreatedAt.IsZero() {
		mission.CreatedAt = now
	}
	if mission.UpdatedAt.IsZero() {
		mission.UpdatedAt = now
	}
	if mission.Status == "" {
		mission.Status = "PENDING"
	}

    var epicID *string
    if mission.EpicID != nil {
        s := mission.EpicID.String()
        epicID = &s
    }

    if r.dbProvider.IsSQLite() {
        _, err := r.dbProvider.Exec(ctx, q, mission.ID.String(), epicID, mission.Title, mission.Status, mission.AssignedAgentID, mission.CreatedAt, mission.UpdatedAt)
        if err != nil {
            return fmt.Errorf("failed to insert mission: %w", err)
        }
        return nil
    }

	_, err := r.dbProvider.Exec(ctx, q, mission.ID, mission.EpicID, mission.Title, mission.Status, mission.AssignedAgentID, mission.CreatedAt, mission.UpdatedAt)
	if err != nil {
		return fmt.Errorf("failed to insert mission: %w", err)
	}

	return nil
}

func (r *missionRepositoryImpl) GetMission(ctx context.Context, id uuid.UUID) (*Mission, error) {
	tableName := r.getTableName("ohc_tasks.missions")
	q := fmt.Sprintf(`SELECT id, epic_id, title, status, assigned_agent_id, created_at, updated_at
		  FROM %s WHERE id = $1`, tableName)

	mission := &Mission{}
	var epicID sql.NullString
    var assignedAgentID sql.NullString
    var missionIDStr string
    var createdAt, updatedAt time.Time

    if r.dbProvider.IsSQLite() {
        err := r.dbProvider.QueryRow(ctx, q, id.String()).Scan(&missionIDStr, &epicID, &mission.Title, &mission.Status, &assignedAgentID, &createdAt, &updatedAt)
        if err != nil {
            if err == sql.ErrNoRows || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
                return nil, nil
            }
            return nil, fmt.Errorf("failed to scan mission: %w", err)
        }
        mission.ID, _ = uuid.Parse(missionIDStr)
    } else {
        err := r.dbProvider.QueryRow(ctx, q, id).Scan(&mission.ID, &epicID, &mission.Title, &mission.Status, &assignedAgentID, &createdAt, &updatedAt)
        if err != nil {
            if err == sql.ErrNoRows || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
                return nil, nil
            }
            return nil, fmt.Errorf("failed to scan mission: %w", err)
        }
    }

    mission.CreatedAt = createdAt
    mission.UpdatedAt = updatedAt

    if epicID.Valid {
        id, _ := uuid.Parse(epicID.String)
        mission.EpicID = &id
    }

    if assignedAgentID.Valid {
        mission.AssignedAgentID = &assignedAgentID.String
    }

	return mission, nil
}

func (r *missionRepositoryImpl) UpdateMissionStatus(ctx context.Context, id uuid.UUID, status string) error {
	tableName := r.getTableName("ohc_tasks.missions")
	q := fmt.Sprintf(`UPDATE %s SET status = $1, updated_at = $2 WHERE id = $3`, tableName)

    if r.dbProvider.IsSQLite() {
        _, err := r.dbProvider.Exec(ctx, q, status, time.Now().UTC(), id.String())
        if err != nil {
            return fmt.Errorf("failed to update mission status: %w", err)
        }
        return nil
    }

	_, err := r.dbProvider.Exec(ctx, q, status, time.Now().UTC(), id)
	if err != nil {
		return fmt.Errorf("failed to update mission status: %w", err)
	}
	return nil
}

func (r *missionRepositoryImpl) AddDependency(ctx context.Context, taskID, dependsOnTaskID uuid.UUID) error {
	tableName := r.getTableName("ohc_tasks.mission_dependencies")
	q := fmt.Sprintf(`INSERT INTO %s (task_id, depends_on_task_id) VALUES ($1, $2)`, tableName)

    if r.dbProvider.IsSQLite() {
        _, err := r.dbProvider.Exec(ctx, q, taskID.String(), dependsOnTaskID.String())
        if err != nil {
            return fmt.Errorf("failed to insert dependency: %w", err)
        }
        return nil
    }

	_, err := r.dbProvider.Exec(ctx, q, taskID, dependsOnTaskID)
	if err != nil {
		return fmt.Errorf("failed to insert dependency: %w", err)
	}
	return nil
}

func (r *missionRepositoryImpl) GetDependencies(ctx context.Context, taskID uuid.UUID) ([]uuid.UUID, error) {
	tableName := r.getTableName("ohc_tasks.mission_dependencies")
	q := fmt.Sprintf(`SELECT depends_on_task_id FROM %s WHERE task_id = $1`, tableName)

    var rows db.Rows
    var err error

    if r.dbProvider.IsSQLite() {
        rows, err = r.dbProvider.Query(ctx, q, taskID.String())
    } else {
        rows, err = r.dbProvider.Query(ctx, q, taskID)
    }

	if err != nil {
		return nil, fmt.Errorf("failed to query dependencies: %w", err)
	}
	defer rows.Close()

	var deps []uuid.UUID
	for rows.Next() {
		if r.dbProvider.IsSQLite() {
            var depIDStr string
            if err := rows.Scan(&depIDStr); err != nil {
                return nil, fmt.Errorf("failed to scan dependency id: %w", err)
            }
            id, _ := uuid.Parse(depIDStr)
            deps = append(deps, id)
        } else {
            var depID uuid.UUID
            if err := rows.Scan(&depID); err != nil {
                return nil, fmt.Errorf("failed to scan dependency id: %w", err)
            }
            deps = append(deps, depID)
        }
	}
	if err = rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}
	return deps, nil
}

// Convert float32 array to a string representation compatible with pgvector if using postgres.
func float32SliceToString(slice []float32) string {
    if len(slice) == 0 {
        return "[]"
    }
    result := "["
    for i, v := range slice {
        if i > 0 {
            result += ","
        }
        result += fmt.Sprintf("%f", v)
    }
    result += "]"
    return result
}


func (r *missionRepositoryImpl) CreateAutodreamVector(ctx context.Context, vector *AutodreamVector) error {
    tableName := r.getTableName("ohc_memory.autodream_vectors")

    // Convert embedding to string for pgvector format
    var emb string
    if len(vector.Embedding) > 0 {
        emb = float32SliceToString(vector.Embedding)
    }

	q := fmt.Sprintf(`INSERT INTO %s (id, task_id, content, embedding, created_at)
		  VALUES ($1, $2, $3, $4, $5)`, tableName)

	if vector.ID == uuid.Nil {
		vector.ID = uuid.New()
	}
	if vector.CreatedAt.IsZero() {
		vector.CreatedAt = time.Now().UTC()
	}

    var err error

    if r.dbProvider.IsSQLite() {
        var taskID *string
        if vector.TaskID != nil {
            s := vector.TaskID.String()
            taskID = &s
        }

        if len(vector.Embedding) > 0 {
            _, err = r.dbProvider.Exec(ctx, q, vector.ID.String(), taskID, vector.Content, emb, vector.CreatedAt)
        } else {
            q2 := fmt.Sprintf(`INSERT INTO %s (id, task_id, content, created_at) VALUES ($1, $2, $3, $4)`, tableName)
            _, err = r.dbProvider.Exec(ctx, q2, vector.ID.String(), taskID, vector.Content, vector.CreatedAt)
        }
    } else {
        if len(vector.Embedding) > 0 {
            _, err = r.dbProvider.Exec(ctx, q, vector.ID, vector.TaskID, vector.Content, emb, vector.CreatedAt)
        } else {
            q2 := fmt.Sprintf(`INSERT INTO %s (id, task_id, content, created_at) VALUES ($1, $2, $3, $4)`, tableName)
            _, err = r.dbProvider.Exec(ctx, q2, vector.ID, vector.TaskID, vector.Content, vector.CreatedAt)
        }
    }

	if err != nil {
		return fmt.Errorf("failed to insert autodream vector: %w", err)
	}
	return nil
}
