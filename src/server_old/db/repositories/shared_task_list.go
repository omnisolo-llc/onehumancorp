package repositories

import (
	"context"
	"database/sql"
	"fmt"
	"sync"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/db/models"
	"github.com/redis/rueidis"
)

type SharedTaskListRepository interface {
	CreateTask(ctx context.Context, task *models.SharedTaskV4) error
	GetTask(ctx context.Context, id string) (*models.SharedTaskV4, error)
	ClaimTask(ctx context.Context, organizationID, agentID string) (*models.SharedTaskV4, error)
	UpdateTaskStatus(ctx context.Context, id, fromState, toState, agentID, reason string) error
}

type SubAgentQueueRepository interface {
	Enqueue(ctx context.Context, job *models.SubAgentJob) error
	ClaimJob(ctx context.Context, organizationID, workerID string) (*models.SubAgentJob, error)
	UpdateJobStatus(ctx context.Context, id, status, workerID, reason string) error
}

type sharedTaskListRepoImpl struct {
	dbProvider  db.Provider
	redisClient rueidis.Client
	mu          sync.Mutex // For SQLite Standalone mode claim synchronization
}

func NewSharedTaskListRepository(dbProvider db.Provider, redisClient rueidis.Client) SharedTaskListRepository {
	return &sharedTaskListRepoImpl{
		dbProvider:  dbProvider,
		redisClient: redisClient,
	}
}

func (r *sharedTaskListRepoImpl) CreateTask(ctx context.Context, task *models.SharedTaskV4) error {
	if task.ID == "" {
		task.ID = uuid.NewString()
	}
	now := time.Now().UTC()
	task.CreatedAt = now
	task.UpdatedAt = now
	if task.Status == "" {
		task.Status = "PENDING"
	}
	if task.Dependencies == "" {
		task.Dependencies = "[]"
	}

	query := `INSERT INTO shared_tasks_v4 (
		id, organization_id, title, description, status, agent_id,
		priority, payload, parent_plan_id, dependencies, created_at, updated_at
	) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)`

	_, err := r.dbProvider.Exec(ctx, query,
		task.ID, task.OrganizationID, task.Title, task.Description, task.Status, task.AgentID,
		task.Priority, task.Payload, task.ParentPlanID, task.Dependencies, task.CreatedAt, task.UpdatedAt,
	)
	return err
}

func (r *sharedTaskListRepoImpl) GetTask(ctx context.Context, id string) (*models.SharedTaskV4, error) {
	query := `SELECT id, organization_id, title, description, status, agent_id,
		priority, payload, parent_plan_id, dependencies, created_at, updated_at
	FROM shared_tasks_v4 WHERE id = $1`

	var task models.SharedTaskV4
	err := r.dbProvider.QueryRow(ctx, query, id).Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AgentID,
		&task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}
	return &task, nil
}

func (r *sharedTaskListRepoImpl) ClaimTask(ctx context.Context, organizationID, agentID string) (*models.SharedTaskV4, error) {
	if r.dbProvider.IsSQLite() {
		r.mu.Lock()
		defer r.mu.Unlock()
	} else if r.redisClient != nil {
		// Use rueidis lock for Cloud mode as requested
		lockKey := "ohc:lock:claim_task:" + organizationID
		cmd := r.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Ex(10 * time.Second).Build()
		err := r.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if !rueidis.IsRedisNil(err) {
				return nil, fmt.Errorf("failed to acquire redis lock: %w", err)
			}
			// Lock held by someone else, but we continue to DB locking for robustness
		} else {
			defer func() {
				delCmd := r.redisClient.B().Del().Key(lockKey).Build()
				_ = r.redisClient.Do(ctx, delCmd)
			}()
		}
	}

	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task models.SharedTaskV4
	var query string

	// DAG Enforcement: Only claim tasks where all dependencies are COMPLETED or SUCCESS
	// We assume dependencies is a JSON array of strings in SQLite/Postgres.
	// For simplicity in this implementation, we check that no dependency exists that is NOT completed.
	// This requires a more complex join or subquery.

	dagFilter := `NOT EXISTS (
		SELECT 1 FROM shared_tasks_v4 dep
		WHERE dep.id IN (SELECT value FROM json_each(t.dependencies))
		AND dep.status NOT IN ('COMPLETED', 'SUCCESS')
	)`

	if !r.dbProvider.IsSQLite() {
		// Postgres uses jsonb_array_elements_text
		dagFilter = `NOT EXISTS (
			SELECT 1 FROM shared_tasks_v4 dep
			WHERE dep.id IN (SELECT jsonb_array_elements_text(t.dependencies::jsonb))
			AND dep.status NOT IN ('COMPLETED', 'SUCCESS')
		)`
	}

	if r.dbProvider.IsSQLite() {
		query = fmt.Sprintf(`UPDATE shared_tasks_v4 AS t
			SET status = 'ASSIGNED', agent_id = $2, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM shared_tasks_v4 AS t
				WHERE status = 'PENDING' AND organization_id = $1
				AND %s
				ORDER BY priority ASC, created_at ASC LIMIT 1
			) RETURNING id, organization_id, title, description, status, agent_id,
				priority, payload, parent_plan_id, dependencies, created_at, updated_at`, dagFilter)

		err = tx.QueryRow(ctx, query, organizationID, agentID).Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AgentID,
			&task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		query = fmt.Sprintf(`SELECT id, organization_id, title, description, status, agent_id,
			priority, payload, parent_plan_id, dependencies, created_at, updated_at
		FROM shared_tasks_v4 AS t
		WHERE status = 'PENDING' AND organization_id = $1
		AND %s
		ORDER BY priority ASC, created_at ASC
		FOR UPDATE SKIP LOCKED LIMIT 1`, dagFilter)

		err = tx.QueryRow(ctx, query, organizationID).Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AgentID,
			&task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt,
		)
		if err == nil {
			updateQuery := `UPDATE shared_tasks_v4 SET status = 'ASSIGNED', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
			_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
			if err == nil {
				task.Status = "ASSIGNED"
				task.AgentID = &agentID
			}
		}
	}

	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}

	// Log transition
	auditQuery := `INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err = tx.Exec(ctx, auditQuery, uuid.NewString(), task.ID, "SHARED_TASK", "PENDING", "ASSIGNED", agentID, "Task claimed by agent")
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return &task, nil
}

func (r *sharedTaskListRepoImpl) UpdateTaskStatus(ctx context.Context, id, fromState, toState, agentID, reason string) error {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `UPDATE shared_tasks_v4 SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND status = $3`
	affected, err := tx.Exec(ctx, query, toState, id, fromState)
	if err != nil {
		return err
	}
	if affected == 0 {
		return fmt.Errorf("task %s not found or status changed from %s", id, fromState)
	}

	auditQuery := `INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err = tx.Exec(ctx, auditQuery, uuid.NewString(), id, "SHARED_TASK", fromState, toState, agentID, reason)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

type subAgentQueueRepoImpl struct {
	dbProvider db.Provider
	mu         sync.Mutex
}

func NewSubAgentQueueRepository(dbProvider db.Provider) SubAgentQueueRepository {
	return &subAgentQueueRepoImpl{
		dbProvider: dbProvider,
	}
}

func (r *subAgentQueueRepoImpl) Enqueue(ctx context.Context, job *models.SubAgentJob) error {
	if job.ID == "" {
		job.ID = uuid.NewString()
	}
	now := time.Now().UTC()
	job.CreatedAt = now
	job.UpdatedAt = now
	if job.Status == "" {
		job.Status = "PENDING"
	}

	query := `INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err := r.dbProvider.Exec(ctx, query, job.ID, job.OrganizationID, job.ParentTaskID, job.Payload, job.Status, job.CreatedAt, job.UpdatedAt)
	return err
}

func (r *subAgentQueueRepoImpl) ClaimJob(ctx context.Context, organizationID, workerID string) (*models.SubAgentJob, error) {
	if r.dbProvider.IsSQLite() {
		r.mu.Lock()
		defer r.mu.Unlock()
	}

	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var job models.SubAgentJob
	var query string

	if r.dbProvider.IsSQLite() {
		query = `UPDATE sub_agent_queue
			SET status = 'IN_PROGRESS', worker_id = $2, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM sub_agent_queue
				WHERE status = 'PENDING' AND organization_id = $1
				ORDER BY created_at ASC LIMIT 1
			) RETURNING id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at`

		err = tx.QueryRow(ctx, query, organizationID, workerID).Scan(
			&job.ID, &job.OrganizationID, &job.ParentTaskID, &job.Payload, &job.Status, &job.WorkerID, &job.CreatedAt, &job.UpdatedAt,
		)
	} else {
		query = `SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
			FROM sub_agent_queue
			WHERE status = 'PENDING' AND organization_id = $1
			ORDER BY created_at ASC
			FOR UPDATE SKIP LOCKED LIMIT 1`

		err = tx.QueryRow(ctx, query, organizationID).Scan(
			&job.ID, &job.OrganizationID, &job.ParentTaskID, &job.Payload, &job.Status, &job.WorkerID, &job.CreatedAt, &job.UpdatedAt,
		)
		if err == nil {
			updateQuery := `UPDATE sub_agent_queue SET status = 'IN_PROGRESS', worker_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
			_, err = tx.Exec(ctx, updateQuery, workerID, job.ID)
			if err == nil {
				job.Status = "IN_PROGRESS"
				job.WorkerID = &workerID
			}
		}
	}

	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}

	// Audit transition for job
	auditQuery := `INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err = tx.Exec(ctx, auditQuery, uuid.NewString(), job.ID, "SUB_AGENT_JOB", "PENDING", "IN_PROGRESS", workerID, "Job claimed by worker")
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return &job, nil
}

func (r *subAgentQueueRepoImpl) UpdateJobStatus(ctx context.Context, id, status, workerID, reason string) error {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Get current status for audit log
	var currentStatus string
	err = tx.QueryRow(ctx, "SELECT status FROM sub_agent_queue WHERE id = $1", id).Scan(&currentStatus)
	if err != nil {
		return err
	}

	query := `UPDATE sub_agent_queue SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	_, err = tx.Exec(ctx, query, status, id)
	if err != nil {
		return err
	}

	auditQuery := `INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`
	_, err = tx.Exec(ctx, auditQuery, uuid.NewString(), id, "SUB_AGENT_JOB", currentStatus, status, workerID, reason)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}
