package orchestration

import (
    "context"
    "encoding/json"
    "errors"
    "fmt"
    "sync"

    "github.com/google/uuid"
    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/memory/autodream"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SharedTaskDB struct {
    ID              string
    ParentPlanID    *string
    OrganizationID  string
    Title           string
    Description     *string
    Status          string
    AgentID *string
    CreatedAt       string
    UpdatedAt       string
}

type SharedTaskOrchestrator struct {
    dbProvider db.Provider
    mu         sync.Mutex
    mesh       MeshTransport
    autodream  autodream.MemoryConsolidator
}

func NewSharedTaskOrchestrator(dbProvider db.Provider, mesh MeshTransport, ad autodream.MemoryConsolidator) *SharedTaskOrchestrator {
    return &SharedTaskOrchestrator{
        dbProvider: dbProvider,
        mesh:       mesh,
        autodream:  ad,
    }
}

func (to *SharedTaskOrchestrator) insertTransition(ctx context.Context, tx db.Tx, taskID, from, toState, agentID, reason string) error {
    id := uuid.New().String()
    _, err := tx.Exec(ctx, `
        INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
        VALUES ($1, $2, 'task', $3, $4, $5, $6, CURRENT_TIMESTAMP)
    `, id, taskID, from, toState, agentID, reason)
    return err
}

func (to *SharedTaskOrchestrator) ClaimTask(ctx context.Context, agentID string) (*SharedTaskDB, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return nil, errors.New("unauthorized: missing claims")
    }

    if to.dbProvider.IsSQLite() {
        if !to.mu.TryLock() {
            telemetry.RecordPostgresLockContention(ctx, "claim_task")
            to.mu.Lock()
        }
        defer to.mu.Unlock()
    }

    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, created_at, updated_at
        FROM shared_tasks
        WHERE status = 'PENDING' AND organization_id = $1
        LIMIT 1
    `
    if !to.dbProvider.IsSQLite() {
        query += " FOR UPDATE SKIP LOCKED"
    }

    var task SharedTaskDB
    err = tx.QueryRow(ctx, query, claims.OrganizationID).Scan(
        &task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title,
        &task.Description, &task.Status, &task.AgentID, &task.CreatedAt, &task.UpdatedAt,
    )

    if err != nil {
        if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            return nil, nil
        }
        return nil, fmt.Errorf("failed to query pending task: %w", err)
    }

    _, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID)
    if err != nil {
        return nil, fmt.Errorf("failed to update task status: %w", err)
    }

    if err := to.insertTransition(ctx, tx, task.ID, "PENDING", "ASSIGNED", agentID, "Task claimed by agent"); err != nil {
        return nil, fmt.Errorf("failed to insert transition: %w", err)
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, fmt.Errorf("failed to commit transaction: %w", err)
    }

    if to.mesh != nil {
        payloadBytes, _ := json.Marshal(map[string]string{"task_id": task.ID})
        _ = to.mesh.BroadcastMeshEvent(context.Background(), "task.assigned", payloadBytes)
    }

    task.Status = "ASSIGNED"
    task.AgentID = &agentID
    return &task, nil
}

func (to *SharedTaskOrchestrator) TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    var current string
    if err := tx.QueryRow(ctx, "SELECT status FROM shared_tasks_master WHERE id = $1", taskID).Scan(&current); err != nil {
        return fmt.Errorf("failed to fetch task %s: %w", taskID, err)
    }

    if current != fromState {
        return fmt.Errorf("task %s is in state %s, expected %s", taskID, current, fromState)
    }

    if _, err := tx.Exec(ctx, "UPDATE shared_tasks_master SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", toState, taskID); err != nil {
        return err
    }

    if err := to.insertTransition(ctx, tx, taskID, fromState, toState, agentID, reason); err != nil {
        return err
    }

    if err := tx.Commit(ctx); err != nil {
        return err
    }

    if toState == "COMPLETED" {
        if to.autodream != nil {
            go func() {
                var payloadText, deliberationLog string
                err := to.dbProvider.QueryRow(context.Background(), "SELECT COALESCE(payload, '{}'), COALESCE(deliberation_log, '{}') FROM shared_tasks_master WHERE id = $1", taskID).Scan(&payloadText, &deliberationLog)
                if err != nil {
                    // Log error here in a real scenario
                    return
                }

                logs := []string{"Task " + taskID + " completed successfully.", "Payload: " + payloadText, "Deliberation Log: " + deliberationLog}
                _ = to.autodream.Consolidate(context.Background(), taskID, logs)
            }()
        }

        if to.mesh != nil {
            go func() {
                payload := map[string]interface{}{
                    "task_id":  taskID,
                    "action":   "COMPLETE",
                    "agent_id": agentID,
                    "status":   "COMPLETED",
                }
                payloadBytes, err := json.Marshal(payload)
                if err == nil {
                    _ = to.mesh.BroadcastMeshEvent(context.Background(), "tasks", payloadBytes)
                }
            }()
        }
    }

    return nil
}


func (to *SharedTaskOrchestrator) ClaimPendingTask(ctx context.Context) (*Task, error) {
    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT id FROM shared_tasks_v2
        WHERE status = 'PENDING'
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `
    var id string
    err = tx.QueryRow(ctx, query).Scan(&id)
    if err != nil {
        return nil, err
    }

    _, err = tx.Exec(ctx, "UPDATE shared_tasks_v2 SET status = 'IN_PROGRESS' WHERE id = $1", id)
    if err != nil {
        return nil, err
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, err
    }

    return &Task{TaskID: id, Status: "IN_PROGRESS"}, nil
}
