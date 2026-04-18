package tasks

import (
    "context"
    "database/sql"
    "errors"
    "sync"
    "time"

    "github.com/google/uuid"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type TaskDecompositionService struct {
    provider db.Provider
    mu       sync.Mutex
}

func NewTaskDecompositionService(p db.Provider) *TaskDecompositionService {
    return &TaskDecompositionService{provider: p}
}

func (s *TaskDecompositionService) CreateTask(ctx context.Context, task *SwarmTask) error {
    if task.ID == "" {
        task.ID = uuid.New().String()
    }
    if task.Dependencies == "" {
        task.Dependencies = "[]"
    }
    query := `INSERT INTO swarm_tasks (id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id, payload, locked_until, created_at)
              VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CURRENT_TIMESTAMP)`
    _, err := s.provider.Exec(ctx, query, task.ID, task.MissionID, task.ParentPlanID, task.Dependencies, task.Title, "PENDING", task.AssignedAgentID, task.Payload, task.LockedUntil)
    return err
}

func (s *TaskDecompositionService) ClaimTask(ctx context.Context, agentID string) (*SwarmTask, error) {
    if s.provider.IsSQLite() {
        s.mu.Lock()
        defer s.mu.Unlock()

        query := `SELECT id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id, payload, locked_until, created_at
                  FROM swarm_tasks
                  WHERE status = 'PENDING'
                  AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                  AND NOT EXISTS (
                      SELECT 1 FROM json_each(dependencies) as dep
                      JOIN swarm_tasks st ON st.id = dep.value
                      WHERE st.status != 'COMPLETED'
                  )
                  LIMIT 1`
        row := s.provider.QueryRow(ctx, query)

        var t SwarmTask
        err := row.Scan(&t.ID, &t.MissionID, &t.ParentPlanID, &t.Dependencies, &t.Title, &t.Status, &t.AssignedAgentID, &t.Payload, &t.LockedUntil, &t.CreatedAt)
        if err != nil {
            if err == sql.ErrNoRows || err.Error() == "sql: no rows in result set" {
                return nil, errors.New("no tasks available")
            }
            return nil, err
        }

        lockedUntil := time.Now().Add(5 * time.Minute)
        updateQuery := `UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = $2 WHERE id = $3`
        _, err = s.provider.Exec(ctx, updateQuery, agentID, lockedUntil, t.ID)
        if err != nil {
            return nil, err
        }

        t.Status = "IN_PROGRESS"
        t.AssignedAgentID = sql.NullString{String: agentID, Valid: true}
        t.LockedUntil = sql.NullTime{Time: lockedUntil, Valid: true}
        return &t, nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return nil, err
    }
    defer tx.Rollback(ctx)

    query := `SELECT id, mission_id, parent_plan_id, dependencies, title, status, assigned_agent_id, payload, locked_until, created_at
              FROM swarm_tasks
              WHERE status = 'PENDING'
              AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
              AND NOT EXISTS (
                  SELECT 1 FROM jsonb_array_elements_text(dependencies) as dep
                  JOIN swarm_tasks st ON st.id::text = dep
                  WHERE st.status != 'COMPLETED'
              )
              FOR UPDATE SKIP LOCKED LIMIT 1`
    row := tx.QueryRow(ctx, query)
    var t SwarmTask
    err = row.Scan(&t.ID, &t.MissionID, &t.ParentPlanID, &t.Dependencies, &t.Title, &t.Status, &t.AssignedAgentID, &t.Payload, &t.LockedUntil, &t.CreatedAt)
    if err != nil {
        if err == sql.ErrNoRows || err.Error() == "sql: no rows in result set" {
            return nil, errors.New("no tasks available")
        }
        return nil, err
    }

    lockedUntil := time.Now().Add(5 * time.Minute)
    updateQuery := `UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = $2 WHERE id = $3`
    _, err = tx.Exec(ctx, updateQuery, agentID, lockedUntil, t.ID)
    if err != nil {
        return nil, err
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, err
    }

    t.Status = "IN_PROGRESS"
    t.AssignedAgentID = sql.NullString{String: agentID, Valid: true}
    t.LockedUntil = sql.NullTime{Time: lockedUntil, Valid: true}
    return &t, nil
}

func (s *TaskDecompositionService) UpdateTaskStatus(ctx context.Context, taskID, fromState, toState, agentID, reason string) error {
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    updateQuery := `UPDATE swarm_tasks SET status = $1 WHERE id = $2 AND status = $3`
    res, err := tx.Exec(ctx, updateQuery, toState, taskID, fromState)
    if err != nil {
        return err
    }

    if res == 0 {
        return errors.New("task not found or state mismatch")
    }

    transID := uuid.New().String()
    insertQuery := `INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
                    VALUES ($1, $2, 'swarm_tasks', $3, $4, $5, $6, CURRENT_TIMESTAMP)`
    _, err = tx.Exec(ctx, insertQuery, transID, taskID, fromState, toState, agentID, reason)
    if err != nil {
        return err
    }

    return tx.Commit(ctx)
}
