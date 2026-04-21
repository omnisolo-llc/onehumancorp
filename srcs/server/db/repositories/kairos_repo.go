package repositories

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type KairosSharedTask struct {
	ID              string    `json:"id" db:"id"`
	OrganizationID  string    `json:"organization_id" db:"organization_id"`
	ParentPlanID    *string   `json:"parent_plan_id" db:"parent_plan_id"`
	Title           string    `json:"title" db:"title"`
	Status          string    `json:"status" db:"status"`
	AssignedAgentID *string   `json:"assigned_agent_id" db:"assigned_agent_id"`
	CreatedAt       time.Time `json:"created_at" db:"created_at"`
	UpdatedAt       time.Time `json:"updated_at" db:"updated_at"`
}

type KairosStateTransition struct {
	ID         string    `json:"id" db:"id"`
	TaskID     string    `json:"task_id" db:"task_id"`
	FromState  string    `json:"from_state" db:"from_state"`
	ToState    string    `json:"to_state" db:"to_state"`
	AgentID    string    `json:"agent_id" db:"agent_id"`
	Reason     *string   `json:"reason" db:"reason"`
	OccurredAt time.Time `json:"occurred_at" db:"occurred_at"`
}

type KairosSubAgentJob struct {
	ID             string          `json:"id" db:"id"`
	OrganizationID string          `json:"organization_id" db:"organization_id"`
	ParentTaskID   *string         `json:"parent_task_id" db:"parent_task_id"`
	Payload        json.RawMessage `json:"payload" db:"payload"`
	Status         string          `json:"status" db:"status"`
	WorkerID       *string         `json:"worker_id" db:"worker_id"`
	CreatedAt      time.Time       `json:"created_at" db:"created_at"`
	UpdatedAt      time.Time       `json:"updated_at" db:"updated_at"`
}

type AutodreamVectorMemory struct {
	ID              string    `json:"id" db:"id"`
	SourceMissionID *string   `json:"source_mission_id" db:"source_mission_id"`
	Content         string    `json:"content" db:"content"`
	Embedding       any       `json:"embedding" db:"embedding"`
	CreatedAt       time.Time `json:"created_at" db:"created_at"`
}

type KairosRepository interface {
	CreateSharedTask(ctx context.Context, task *KairosSharedTask) error
	CreateStateTransition(ctx context.Context, transition *KairosStateTransition) error
	CreateSubAgentJob(ctx context.Context, job *KairosSubAgentJob) error
	CreateVectorMemory(ctx context.Context, memory *AutodreamVectorMemory) error
}

type kairosRepo struct {
	provider db.Provider
}

func NewKairosRepository(provider db.Provider) KairosRepository {
	return &kairosRepo{provider: provider}
}

func (r *kairosRepo) CreateSharedTask(ctx context.Context, task *KairosSharedTask) error {
	query := `INSERT INTO kairos_shared_tasks (id, organization_id, parent_plan_id, title, status, assigned_agent_id)
		VALUES ($1, $2, $3, $4, $5, $6)`

	_, err := r.provider.Exec(ctx, query, task.ID, task.OrganizationID, task.ParentPlanID, task.Title, task.Status, task.AssignedAgentID)
	return err
}

func (r *kairosRepo) CreateStateTransition(ctx context.Context, transition *KairosStateTransition) error {
	query := `INSERT INTO kairos_state_transitions (id, task_id, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, $3, $4, $5, $6)`

	_, err := r.provider.Exec(ctx, query, transition.ID, transition.TaskID, transition.FromState, transition.ToState, transition.AgentID, transition.Reason)
	return err
}

func (r *kairosRepo) CreateSubAgentJob(ctx context.Context, job *KairosSubAgentJob) error {
	query := `INSERT INTO kairos_sub_agent_jobs (id, organization_id, parent_task_id, payload, status, worker_id)
		VALUES ($1, $2, $3, $4, $5, $6)`

	_, err := r.provider.Exec(ctx, query, job.ID, job.OrganizationID, job.ParentTaskID, job.Payload, job.Status, job.WorkerID)
	return err
}

func (r *kairosRepo) CreateVectorMemory(ctx context.Context, memory *AutodreamVectorMemory) error {
	query := `INSERT INTO autodream_vector_memories (id, source_mission_id, content, embedding)
		VALUES ($1, $2, $3, $4)`

	_, err := r.provider.Exec(ctx, query, memory.ID, memory.SourceMissionID, memory.Content, memory.Embedding)
	return err
}
