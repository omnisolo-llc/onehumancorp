package orchestration

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SharedTaskDB struct {
	db db.Provider
	mu sync.Mutex // Application-level mutex for SQLite standalone mode
}

func NewSharedTaskDB(db db.Provider) *SharedTaskDB {
	return &SharedTaskDB{db: db}
}

// ClaimTask attempts to claim a task.
func (tdb *SharedTaskDB) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, fmt.Errorf("unauthorized: missing claims")
	}

	if tdb.db.IsSQLite() {
		// Use application-level mutex for SQLite standalone mode
		tdb.mu.Lock()
		defer tdb.mu.Unlock()

		tx, err := tdb.db.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		// Find a pending task
		query := `SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, created_at, updated_at
		          FROM shared_tasks
		          WHERE status = 'PENDING' AND organization_id = $1 LIMIT 1`

		row := tx.QueryRow(ctx, query, claims.OrganizationID)
		task := &SharedTask{}
		var parentPlanID *string
		var assignedAgentID *string
		var description *string
		var createdAt, updatedAt time.Time

		err = row.Scan(&task.ID, &task.OrganizationID, &parentPlanID, &task.Title, &description, &task.Status, &assignedAgentID, &createdAt, &updatedAt)
		if err != nil {
			// e.g. sql.ErrNoRows if nothing to claim
			return nil, err // Let caller handle sql.ErrNoRows or pgx.ErrNoRows
		}

		if parentPlanID != nil {
			task.ParentPlanID = *parentPlanID
		}
		if assignedAgentID != nil {
			task.AssignedAgentID = *assignedAgentID
		}
		if description != nil {
			task.Description = *description
		}
		task.CreatedAt = createdAt
		task.UpdatedAt = updatedAt

		// Update to ASSIGNED
		updateQuery := `UPDATE shared_tasks SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
		if err != nil {
			return nil, err
		}

		if err := tx.Commit(ctx); err != nil {
			return nil, err
		}

		task.Status = "ASSIGNED"
		task.AssignedAgentID = agentID
		return task, nil
	} else {
		// Use FOR UPDATE SKIP LOCKED for PostgreSQL
		tx, err := tdb.db.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		query := `SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, created_at, updated_at
		          FROM shared_tasks
		          WHERE status = 'PENDING' AND organization_id = $1
		          FOR UPDATE SKIP LOCKED LIMIT 1`

		row := tx.QueryRow(ctx, query, claims.OrganizationID)
		task := &SharedTask{}
		var parentPlanID *string
		var assignedAgentID *string
		var description *string
		var createdAt, updatedAt time.Time

		err = row.Scan(&task.ID, &task.OrganizationID, &parentPlanID, &task.Title, &description, &task.Status, &assignedAgentID, &createdAt, &updatedAt)
		if err != nil {
			return nil, err
		}

		if parentPlanID != nil {
			task.ParentPlanID = *parentPlanID
		}
		if assignedAgentID != nil {
			task.AssignedAgentID = *assignedAgentID
		}
		if description != nil {
			task.Description = *description
		}
		task.CreatedAt = createdAt
		task.UpdatedAt = updatedAt

		// Update to ASSIGNED
		updateQuery := `UPDATE shared_tasks SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
		if err != nil {
			return nil, err
		}

		if err := tx.Commit(ctx); err != nil {
			return nil, err
		}

		task.Status = "ASSIGNED"
		task.AssignedAgentID = agentID
		return task, nil
	}
}
