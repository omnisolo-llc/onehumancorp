package orchestration

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"os"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func broadcastMeshEvent(action string, taskID string, priority string, agentID string) {
	baseURL := os.Getenv("OHC_SERVER_URL")
	if baseURL == "" {
		baseURL = "http://localhost:8080"
	}
	url := fmt.Sprintf("%s/api/mesh/broadcast", baseURL)

	payload := map[string]interface{}{
		"agent_id": "kairos-orchestrator-1",
		"channel":  "orchestration.tasks",
		"action":   action,
		"status":   "SUCCESS",
		"payload": map[string]interface{}{
			"task_id":   taskID,
			"priority":  priority,
			"timestamp": time.Now().UTC().Format(time.RFC3339),
		},
	}

	if agentID != "" {
		payload["agent_id"] = agentID
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		return
	}

	go func() {
		req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
		if err != nil {
			return
		}
		req.Header.Set("Content-Type", "application/json")

		client := &http.Client{Timeout: 5 * time.Second}
		resp, err := client.Do(req)
		if err == nil {
			resp.Body.Close()
		}
	}()
}

type Mission struct {
	ID             string
	OrganizationID string
	Tasks          []Task
}

type Task struct {
	ID              string
	OrganizationID  string
	Title           string
	Description     string
	Status          string
	AssignedAgentID *string
	Priority        string
	Payload         string
	ParentPlanID    *string
	Dependencies    string
	LockedUntil     *time.Time
}

type TaskStore interface {
	DecomposeMission(ctx context.Context, m Mission) ([]Task, error)
	ClaimNextTask(ctx context.Context, agentID string) (*Task, error)
}

type DefaultTaskStore struct {
	mu sync.Mutex
	db db.Provider
}

func NewTaskStore(provider db.Provider) TaskStore {
	return &DefaultTaskStore{db: provider}
}

func (s *DefaultTaskStore) DecomposeMission(ctx context.Context, m Mission) ([]Task, error) {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	for _, t := range m.Tasks {
		query := `INSERT INTO shared_tasks_v2 (id, organization_id, title, description, status, priority, payload, parent_plan_id, dependencies)
		VALUES ($1, $2, $3, $4, 'PENDING', $5, $6, $7, $8)`

		_, err := tx.Exec(ctx, query, t.ID, t.OrganizationID, t.Title, t.Description, t.Priority, t.Payload, t.ParentPlanID, t.Dependencies)
		if err != nil {
			return nil, err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}
	for _, t := range m.Tasks {
		broadcastMeshEvent("TASK_DECOMPOSED", t.ID, t.Priority, "")
	}

	return m.Tasks, nil
}

func (s *DefaultTaskStore) ClaimNextTask(ctx context.Context, agentID string) (*Task, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return s.claimSQLite(ctx, agentID)
	}
	return s.claimPostgres(ctx, agentID)
}

func (s *DefaultTaskStore) claimPostgres(ctx context.Context, agentID string) (*Task, error) {
	query := `
		UPDATE shared_tasks_v2
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = NOW() + INTERVAL '1 hour', updated_at = NOW()
		WHERE id = (
			SELECT id FROM shared_tasks_v2
			WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < NOW())
			ORDER BY created_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies, locked_until
	`
	var t Task
	err := s.db.QueryRow(ctx, query, agentID).Scan(
		&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Status, &t.AssignedAgentID, &t.Priority, &t.Payload, &t.ParentPlanID, &t.Dependencies, &t.LockedUntil,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}
	broadcastMeshEvent("TASK_CLAIMED", t.ID, t.Priority, agentID)

	return &t, nil
}

func (s *DefaultTaskStore) claimSQLite(ctx context.Context, agentID string) (*Task, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var t Task
	query := `SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies, locked_until
	FROM shared_tasks_v2
	WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < datetime('now'))
	ORDER BY created_at ASC LIMIT 1`

	err = tx.QueryRow(ctx, query).Scan(
		&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Status, &t.AssignedAgentID, &t.Priority, &t.Payload, &t.ParentPlanID, &t.Dependencies, &t.LockedUntil,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	updateQuery := `UPDATE shared_tasks_v2 SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = datetime('now', '+1 hour'), updated_at = datetime('now') WHERE id = $2`
	_, err = tx.Exec(ctx, updateQuery, agentID, t.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	t.Status = "IN_PROGRESS"
	t.AssignedAgentID = &agentID
	broadcastMeshEvent("TASK_CLAIMED", t.ID, t.Priority, agentID)

	return &t, nil
}
