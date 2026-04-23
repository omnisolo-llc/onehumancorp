package kairos

import (
	"context"
	"encoding/json"
	"time"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// SharedTask models the shared_tasks phase1 database table.
type SharedTask struct {
	ID        string          `json:"id"`
	AgentID   string          `json:"agent_id"`
	Status    string          `json:"status"`
	Payload   json.RawMessage `json:"payload"`
	CreatedAt time.Time       `json:"created_at"`
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

// ActionRisk defines the risk level of an agent action.
type ActionRisk string

const (
	ActionRiskLow  ActionRisk = "Low"
	ActionRiskHigh ActionRisk = "High"
)

// ApprovalStatus represents the human-in-the-loop review state.
type ApprovalStatus string

const (
	ApprovalStatusPending  ApprovalStatus = "Pending"
	ApprovalStatusApproved ApprovalStatus = "Approved"
	ApprovalStatusRejected ApprovalStatus = "Rejected"
)

// AgentTaskPayload extends the SharedTask payload for high-risk actions.
type AgentTaskPayload struct {
	ActionRisk      ActionRisk      `json:"action_risk,omitempty"`
	ProposedContent string          `json:"proposed_content,omitempty"`
	ApprovalStatus  ApprovalStatus  `json:"approval_status,omitempty"`
}

func (r *SharedTaskRepo) UpdateStatus(ctx context.Context, id string, status string) error {
	query := `UPDATE shared_tasks SET status = $1 WHERE id = $2`
	_, err := r.provider.Exec(ctx, query, status, id)
	return err
}

func (r *SharedTaskRepo) GetPendingReview(ctx context.Context) ([]*SharedTask, error) {
	query := `SELECT id, agent_id, status, payload, created_at FROM shared_tasks WHERE status = 'Pending'`
	rows, err := r.provider.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*SharedTask
	for rows.Next() {
		var task SharedTask
		var payloadStr string
		var createdAt string
		if r.provider.IsSQLite() {
			if err := rows.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &createdAt); err != nil {
				return nil, err
			}
			task.CreatedAt, _ = time.Parse(time.RFC3339, createdAt)
		} else {
			var t time.Time
			if err := rows.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &t); err != nil {
				return nil, err
			}
			task.CreatedAt = t
		}
		if payloadStr != "" {
			task.Payload = json.RawMessage(payloadStr)
		}
		tasks = append(tasks, &task)
	}
	return tasks, nil
}
