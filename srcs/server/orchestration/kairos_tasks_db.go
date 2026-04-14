package orchestration

import (
    "context"
    "errors"
    "fmt"
    "sync"
    "database/sql"
    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type KairosSharedTask struct {
    ID             string
    OrganizationID string
    Title          string
    Description    *string
    Status         string
    AgentID        *string
    Priority       string
    Payload        *string
    ParentPlanID   *string
    Dependencies   string
    CreatedAt      string
    UpdatedAt      string
}

type KairosTaskOrchestrator struct {
    dbProvider db.Provider
    mu         sync.Mutex
}

func NewKairosTaskOrchestrator(dbProvider db.Provider) *KairosTaskOrchestrator {
    return &KairosTaskOrchestrator{
        dbProvider: dbProvider,
    }
}

func (to *KairosTaskOrchestrator) ClaimTask(ctx context.Context, agentID string) (*KairosSharedTask, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return nil, errors.New("unauthorized: missing claims")
    }

    if to.dbProvider.IsSQLite() {
        return to.claimTaskSQLite(ctx, claims.OrganizationID, agentID)
    }
    return to.claimTaskPostgres(ctx, claims.OrganizationID, agentID)
}

func (to *KairosTaskOrchestrator) claimTaskSQLite(ctx context.Context, orgID, agentID string) (*KairosSharedTask, error) {
    to.mu.Lock()
    defer to.mu.Unlock()

    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
        FROM shared_tasks_kairos
        WHERE status = 'PENDING' AND organization_id = $1
        LIMIT 1
    `
    row := tx.QueryRow(ctx, query, orgID)

    var task KairosSharedTask
    if err := row.Scan(
        &task.ID, &task.OrganizationID, &task.Title, &task.Description,
        &task.Status, &task.AgentID, &task.Priority, &task.Payload,
        &task.ParentPlanID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt,
    ); err != nil {
        if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            return nil, nil
        }
        return nil, fmt.Errorf("failed to query pending task: %w", err)
    }

    if _, err = tx.Exec(ctx, "UPDATE shared_tasks_kairos SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
        return nil, fmt.Errorf("failed to update task status: %w", err)
    }

    task.UpdatedAt = "CURRENT_TIMESTAMP"

    if err := tx.Commit(ctx); err != nil {
        return nil, fmt.Errorf("failed to commit transaction: %w", err)
    }

    task.Status = "IN_PROGRESS"
    task.AgentID = &agentID
    return &task, nil
}

func (to *KairosTaskOrchestrator) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*KairosSharedTask, error) {
    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
        FROM shared_tasks_kairos
        WHERE status = 'PENDING' AND organization_id = $1
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `
    row := tx.QueryRow(ctx, query, orgID)

    var task KairosSharedTask
    if err := row.Scan(
        &task.ID, &task.OrganizationID, &task.Title, &task.Description,
        &task.Status, &task.AgentID, &task.Priority, &task.Payload,
        &task.ParentPlanID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt,
    ); err != nil {
        if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            return nil, nil
        }
        return nil, fmt.Errorf("failed to query pending task: %w", err)
    }

    if _, err = tx.Exec(ctx, "UPDATE shared_tasks_kairos SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
        return nil, fmt.Errorf("failed to update task status: %w", err)
    }

    task.UpdatedAt = "CURRENT_TIMESTAMP"

    if err := tx.Commit(ctx); err != nil {
        return nil, fmt.Errorf("failed to commit transaction: %w", err)
    }

    task.Status = "IN_PROGRESS"
    task.AgentID = &agentID
    return &task, nil
}
