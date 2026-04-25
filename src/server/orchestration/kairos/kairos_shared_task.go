package kairos

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
	"github.com/onehumancorp/mono/src/server/db"
)

// SharedTask models the shared_tasks phase1 database table.
type SharedTask struct {
	ID        string          `json:"id"`
	AgentID   string          `json:"agent_id"`
	Status    string          `json:"status"`
	Payload   json.RawMessage `json:"payload"`
	CreatedAt time.Time       `json:"created_at"`
	ProposedContent string          `json:"proposed_content,omitempty"`
}

type SharedTaskRepo struct {
    provider db.Provider
}

func NewSharedTaskRepo(provider db.Provider) *SharedTaskRepo {
    return &SharedTaskRepo{provider: provider}
}

func (r *SharedTaskRepo) Insert(ctx context.Context, task *SharedTask) error {
    query := `INSERT INTO shared_tasks (id, agent_id, status, payload, created_at) VALUES ($1, $2, $3, $4, $5)`
    payloadStr := "{}"
    if task.Payload != nil && len(task.Payload) > 0 {
        payloadStr = string(task.Payload)
    }
    createdAt := task.CreatedAt.Format(time.RFC3339)
    _, err := r.provider.Exec(ctx, query, task.ID, task.AgentID, task.Status, payloadStr, createdAt)
    return err
}

func (r *SharedTaskRepo) Get(ctx context.Context, id string) (*SharedTask, error) {
    query := `SELECT id, agent_id, status, payload, created_at FROM shared_tasks WHERE id = $1`
    var task SharedTask
    var payloadStr string
    var createdAt string

    row := r.provider.QueryRow(ctx, query, id)
    if r.provider.IsSQLite() {
        err := row.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &createdAt)
        if err != nil {
            return nil, err
        }
        task.CreatedAt, _ = time.Parse(time.RFC3339, createdAt)
    } else {
        var t time.Time
        err := row.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &t)
        if err != nil {
            return nil, err
        }
        task.CreatedAt = t
    }

    if payloadStr != "" {
        task.Payload = json.RawMessage(payloadStr)
    }
    return &task, nil
}

func (r *SharedTaskRepo) GetPendingApprovals(ctx context.Context, organizationID, agentID string) ([]*SharedTask, error) {
    query := `SELECT id, agent_id, status, COALESCE(payload, '{}'), created_at, COALESCE(proposed_content, '') FROM shared_tasks WHERE organization_id = $1 AND agent_id = $2 AND approval_status = 'Pending' AND action_risk = 'High'`
    rows, err := r.provider.Query(ctx, query, organizationID, agentID)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var tasks []*SharedTask
    for rows.Next() {
        var task SharedTask
        var payloadStr string
        var createdAtStr string
        var proposedContent string

        if r.provider.IsSQLite() {
            if err := rows.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &createdAtStr, &proposedContent); err != nil {
                return nil, err
            }
            task.CreatedAt, _ = time.Parse(time.RFC3339, createdAtStr)
        } else {
            var t time.Time
            if err := rows.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &t, &proposedContent); err != nil {
                return nil, err
            }
            task.CreatedAt = t
        }

        if payloadStr != "" {
            task.Payload = json.RawMessage(payloadStr)
        }
        task.ProposedContent = proposedContent
        tasks = append(tasks, &task)
    }
    return tasks, nil
}

func (r *SharedTaskRepo) UpdateApprovalStatus(ctx context.Context, organizationID, taskID, approvalStatus, newStatus string) error {
    query := `UPDATE shared_tasks SET approval_status = $1, status = $2 WHERE organization_id = $3 AND id = $4 AND approval_status = 'Pending'`
    res, err := r.provider.Exec(ctx, query, approvalStatus, newStatus, organizationID, taskID)
    if err != nil {
        return err
    }
    if res == 0 {
        return fmt.Errorf("task not found or not pending approval")
    }
    return nil
}
