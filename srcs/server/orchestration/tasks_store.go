package orchestration

import (
	"context"
	"github.com/onehumancorp/mono/srcs/server/db"
	"sync"
	"errors"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

type TaskStore interface {
	ClaimTask(ctx context.Context, agentID string) (*SharedTaskDB, error)
	TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error
}

func NewTaskStore(dbProvider db.Provider) TaskStore {
	return NewSharedTaskOrchestrator(dbProvider)
}



type DecompositionTaskStore interface {
    ClaimTask(ctx context.Context, agentID string) (*SharedTaskDecomposition, error)
    TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error
}

type decompositionStore struct {
    dbProvider db.Provider
    mu         sync.Mutex
}

func NewDecompositionTaskStore(dbProvider db.Provider) DecompositionTaskStore {
    return &decompositionStore{dbProvider: dbProvider}
}

func (ds *decompositionStore) ClaimTask(ctx context.Context, agentID string) (*SharedTaskDecomposition, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil { return nil, errors.New("unauthorized: missing claims") }
    if ds.dbProvider.IsSQLite() {
        ds.mu.Lock()
        defer ds.mu.Unlock()
    }
    tx, err := ds.dbProvider.Begin(ctx)
    if err != nil { return nil, err }
    defer tx.Rollback(ctx)

    query := `SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at FROM shared_tasks_decomposition WHERE status = 'PENDING' AND organization_id = $1 LIMIT 1`
    if !ds.dbProvider.IsSQLite() {
        query += " FOR UPDATE SKIP LOCKED"
    }
    row := tx.QueryRow(ctx, query, claims.OrganizationID)
    var task SharedTaskDecomposition
    var payloadBytes, depsBytes []byte
    if err := row.Scan(&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID, &task.Priority, &payloadBytes, &task.ParentPlanID, &depsBytes, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt); err != nil {
        if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" { return nil, nil }
        return nil, err
    }
    if len(payloadBytes) > 0 { task.Payload = payloadBytes }
    if len(depsBytes) > 0 { task.Dependencies = depsBytes }

    if _, err := tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
        return nil, err
    }
    if err := tx.Commit(ctx); err != nil { return nil, err }
    task.Status = "IN_PROGRESS"
    task.AssignedAgentID = &agentID
    return &task, nil
}

func (ds *decompositionStore) TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
    tx, err := ds.dbProvider.Begin(ctx)
    if err != nil { return err }
    defer tx.Rollback(ctx)
    res, err := tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND status = $3", toState, taskID, fromState)
    if err != nil { return err }
    if res == 0 { return fmt.Errorf("task %s state transition from %s failed or task not found", taskID, fromState) }
    return tx.Commit(ctx)
}
