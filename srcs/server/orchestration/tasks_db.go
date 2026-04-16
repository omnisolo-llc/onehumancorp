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

// Task matches the design doc requirement
type Task struct {
	ID string
	Payload string
	Status string
}

func (to *SharedTaskOrchestrator) ClaimPendingTask(ctx context.Context) (*Task, error) {
	tx, err := to.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		SELECT st.id, st.payload, st.status
		FROM shared_tasks_decomposition st
		WHERE st.status = 'PENDING'
		LIMIT 1
		FOR UPDATE SKIP LOCKED
	`

	if to.dbProvider.IsSQLite() {
		query = `
			SELECT st.id, st.payload, st.status
			FROM shared_tasks_decomposition st
			WHERE st.status = 'PENDING'
			LIMIT 1
		`
		to.mu.Lock()
		defer to.mu.Unlock()
	}

	row := tx.QueryRow(ctx, query)
	var task Task
	var payloadStr *string
	if err := row.Scan(&task.ID, &payloadStr, &task.Status); err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}
	if payloadStr != nil {
		task.Payload = *payloadStr
	}

	if _, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = $1", task.ID); err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	return &task, nil
}

func (to *SharedTaskOrchestrator) ClaimTask(ctx context.Context, agentID string) (*SharedTaskDB, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return nil, errors.New("unauthorized: missing claims")
    }

    if to.dbProvider.IsSQLite() {
        return to.claimTaskSQLite(ctx, claims.OrganizationID, agentID)
    }
    return to.claimTaskPostgres(ctx, claims.OrganizationID, agentID)
}

func (to *SharedTaskOrchestrator) claimTaskSQLite(ctx context.Context, orgID, agentID string) (*SharedTaskDB, error) {
    if !to.mu.TryLock() {
        telemetry.RecordPostgresLockContention(ctx, "claim_task")
        to.mu.Lock()
    }
    defer to.mu.Unlock()

    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT st.id, st.organization_id, st.parent_plan_id, st.title, st.description, st.status, st.assigned_agent_id, st.created_at, st.updated_at
        FROM shared_tasks_decomposition st
        WHERE st.status = 'PENDING' AND st.organization_id = $1
        AND NOT EXISTS (
            SELECT 1 FROM json_each(st.dependencies) AS dep
            JOIN shared_tasks_decomposition d ON d.id = dep.value
            WHERE d.status != 'COMPLETED'
        )
        LIMIT 1
    `
    row := tx.QueryRow(ctx, query, orgID)

    var task SharedTaskDB
    if err := row.Scan(
        &task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title,
        &task.Description, &task.Status, &task.AgentID, &task.CreatedAt, &task.UpdatedAt,
    ); err != nil {
        if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            var checkID string
            checkQuery := `
                SELECT st.id FROM shared_tasks_decomposition st
                WHERE st.status = 'PENDING' AND st.organization_id = $1
                AND NOT EXISTS (
                    SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep
                    JOIN shared_tasks_decomposition d ON d.id::text = dep
                    WHERE d.status != 'COMPLETED'
                )
                LIMIT 1
            `
            if checkErr := tx.QueryRow(ctx, checkQuery, orgID).Scan(&checkID); checkErr == nil && checkID != "" {
                telemetry.RecordPostgresLockContention(ctx, "claim_task")
            }
            return nil, nil
        }
        return nil, fmt.Errorf("failed to query pending task: %w", err)
    }

    if _, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
        return nil, fmt.Errorf("failed to update task status: %w", err)
    }

    if err := to.insertTransition(ctx, tx, task.ID, "PENDING", "IN_PROGRESS", agentID, "Task claimed by agent"); err != nil {
        return nil, fmt.Errorf("failed to insert transition: %w", err)
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, fmt.Errorf("failed to commit transaction: %w", err)
    }

    task.Status = "IN_PROGRESS"
    task.AgentID = &agentID
    return &task, nil
}

func (to *SharedTaskOrchestrator) claimTaskPostgres(ctx context.Context, orgID, agentID string) (*SharedTaskDB, error) {
    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT st.id, st.organization_id, st.parent_plan_id, st.title, st.description, st.status, st.assigned_agent_id, st.created_at, st.updated_at
        FROM shared_tasks_decomposition st
        WHERE st.status = 'PENDING' AND st.organization_id = $1
        AND NOT EXISTS (
            SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep
            JOIN shared_tasks_decomposition d ON d.id::text = dep
            WHERE d.status != 'COMPLETED'
        )
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `
    row := tx.QueryRow(ctx, query, orgID)

    var task SharedTaskDB
    if err := row.Scan(
        &task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title,
        &task.Description, &task.Status, &task.AgentID, &task.CreatedAt, &task.UpdatedAt,
    ); err != nil {
        if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            var checkID string
            checkQuery := `
                SELECT st.id FROM shared_tasks_decomposition st
                WHERE st.status = 'PENDING' AND st.organization_id = $1
                AND NOT EXISTS (
                    SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep
                    JOIN shared_tasks_decomposition d ON d.id::text = dep
                    WHERE d.status != 'COMPLETED'
                )
                LIMIT 1
            `
            if checkErr := tx.QueryRow(ctx, checkQuery, orgID).Scan(&checkID); checkErr == nil && checkID != "" {
                telemetry.RecordPostgresLockContention(ctx, "claim_task")
            }
            return nil, nil
        }
        return nil, fmt.Errorf("failed to query pending task: %w", err)
    }

    if _, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", agentID, task.ID); err != nil {
        return nil, fmt.Errorf("failed to update task status: %w", err)
    }

    if err := to.insertTransition(ctx, tx, task.ID, "PENDING", "IN_PROGRESS", agentID, "Task claimed by agent"); err != nil {
        return nil, fmt.Errorf("failed to insert transition: %w", err)
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, fmt.Errorf("failed to commit transaction: %w", err)
    }

    task.Status = "IN_PROGRESS"
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
    if err := tx.QueryRow(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = $1", taskID).Scan(&current); err != nil {
        return fmt.Errorf("failed to fetch task %s: %w", taskID, err)
    }

    if current != fromState {
        return fmt.Errorf("task %s is in state %s, expected %s", taskID, current, fromState)
    }

    if _, err := tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", toState, taskID); err != nil {
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
                var query string
                if to.dbProvider.IsSQLite() {
                    query = "SELECT COALESCE(payload, '{}') FROM shared_tasks_decomposition WHERE id = $1"
                } else {
                    query = "SELECT COALESCE(payload::text, '{}') FROM shared_tasks_decomposition WHERE id = $1"
                }
                err := to.dbProvider.QueryRow(context.Background(), query, taskID).Scan(&payloadText)
                deliberationLog = "{}" // Add when deliberation log column exists

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
