package kairos

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SharedTask models the shared_tasks phase1 database table.
type SharedTask struct {
	ID              string          `json:"id"`
	AgentID         string          `json:"agent_id"`
	Status          string          `json:"status"`
	Payload         json.RawMessage `json:"payload"`
	ActionRisk      string          `json:"action_risk,omitempty"`
	ApprovalStatus  string          `json:"approval_status,omitempty"`
	ProposedContent string          `json:"proposed_content,omitempty"`
	CreatedAt       time.Time       `json:"created_at"`
}

type SharedTaskRepo struct {
	provider db.Provider
}

func NewSharedTaskRepo(provider db.Provider) *SharedTaskRepo {
	return &SharedTaskRepo{provider: provider}
}

func (r *SharedTaskRepo) Insert(ctx context.Context, task *SharedTask) error {
	query := `INSERT INTO shared_tasks (id, agent_id, status, payload, action_risk, approval_status, proposed_content, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`
	payloadStr := "{}"
	if task.Payload != nil && len(task.Payload) > 0 {
		payloadStr = string(task.Payload)
	}

	// Default approval status
	if task.ApprovalStatus == "" {
		task.ApprovalStatus = "PENDING"
	}

	createdAt := task.CreatedAt.Format(time.RFC3339)
	var actionRisk sql.NullString
	if task.ActionRisk != "" {
		actionRisk.String = task.ActionRisk
		actionRisk.Valid = true
	}
	var approvalStatus sql.NullString
	if task.ApprovalStatus != "" {
		approvalStatus.String = task.ApprovalStatus
		approvalStatus.Valid = true
	}
	var proposedContent sql.NullString
	if task.ProposedContent != "" {
		proposedContent.String = task.ProposedContent
		proposedContent.Valid = true
	}

	_, err := r.provider.Exec(ctx, query, task.ID, task.AgentID, task.Status, payloadStr, actionRisk, approvalStatus, proposedContent, createdAt)
	return err
}

func (r *SharedTaskRepo) Get(ctx context.Context, id string) (*SharedTask, error) {
	query := `SELECT id, agent_id, status, payload, action_risk, approval_status, proposed_content, created_at FROM shared_tasks WHERE id = $1`
	var task SharedTask
	var payloadStr string
	var createdAt string
	var actionRisk, approvalStatus, proposedContent sql.NullString

	row := r.provider.QueryRow(ctx, query, id)
	if r.provider.IsSQLite() {
		err := row.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &actionRisk, &approvalStatus, &proposedContent, &createdAt)
		if err != nil {
			return nil, err
		}
		task.CreatedAt, _ = time.Parse(time.RFC3339, createdAt)
	} else {
		var t time.Time
		err := row.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &actionRisk, &approvalStatus, &proposedContent, &t)
		if err != nil {
			return nil, err
		}
		task.CreatedAt = t
	}

	if payloadStr != "" {
		task.Payload = json.RawMessage(payloadStr)
	}
	if actionRisk.Valid {
		task.ActionRisk = actionRisk.String
	}
	if approvalStatus.Valid {
		task.ApprovalStatus = approvalStatus.String
	}
	if proposedContent.Valid {
		task.ProposedContent = proposedContent.String
	}
	return &task, nil
}

func (r *SharedTaskRepo) GetPendingApprovals(ctx context.Context) ([]SharedTask, error) {
	query := `SELECT id, agent_id, status, payload, action_risk, approval_status, proposed_content, created_at FROM shared_tasks WHERE approval_status = 'PENDING' AND action_risk = 'High'`
	rows, err := r.provider.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []SharedTask
	for rows.Next() {
		var task SharedTask
		var payloadStr string
		var createdAt string
		var actionRisk, approvalStatus, proposedContent sql.NullString

		if r.provider.IsSQLite() {
			err := rows.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &actionRisk, &approvalStatus, &proposedContent, &createdAt)
			if err != nil {
				return nil, err
			}
			task.CreatedAt, _ = time.Parse(time.RFC3339, createdAt)
		} else {
			var t time.Time
			err := rows.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &actionRisk, &approvalStatus, &proposedContent, &t)
			if err != nil {
				return nil, err
			}
			task.CreatedAt = t
		}

		if payloadStr != "" {
			task.Payload = json.RawMessage(payloadStr)
		}
		if actionRisk.Valid {
			task.ActionRisk = actionRisk.String
		}
		if approvalStatus.Valid {
			task.ApprovalStatus = approvalStatus.String
		}
		if proposedContent.Valid {
			task.ProposedContent = proposedContent.String
		}
		tasks = append(tasks, task)
	}
	return tasks, nil
}

func (r *SharedTaskRepo) UpdateApprovalStatus(ctx context.Context, id, status string) error {
	query := `UPDATE shared_tasks SET approval_status = $1 WHERE id = $2`
	_, err := r.provider.Exec(ctx, query, status, id)
	return err
}
