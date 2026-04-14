package tasks

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
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

type Task struct {
	ID              string
	OrganizationID  string
	Title           string
	Description     string
	Status          string
	AssignedAgentID string
	Priority        string
	Payload         string
	ParentPlanID    string
	Dependencies    string
	LockedUntil     time.Time
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

type Mission struct {
	OrganizationID string
	Title          string
	Description    string
	Priority       string
	Payload        string
	ParentPlanID   string
	Dependencies   []string
}

type TaskStore interface {
	DecomposeMission(ctx context.Context, mission Mission) ([]Task, error)
	ClaimNextTask(ctx context.Context, agentID string) (*Task, error)
}

type defaultTaskStore struct {
	mu sync.Mutex
	db db.Provider
}

func NewTaskStore(provider db.Provider) TaskStore {
	return &defaultTaskStore{
		db: provider,
	}
}

func (s *defaultTaskStore) DecomposeMission(ctx context.Context, mission Mission) ([]Task, error) {
	// Create a single task for the mission
	id := fmt.Sprintf("task-%d", time.Now().UnixNano())
	now := time.Now()

	depsBytes, _ := json.Marshal(mission.Dependencies)

	task := Task{
		ID:             id,
		OrganizationID: mission.OrganizationID,
		Title:          mission.Title,
		Description:    mission.Description,
		Status:         "PENDING",
		Priority:       mission.Priority,
		Payload:        mission.Payload,
		ParentPlanID:   mission.ParentPlanID,
		Dependencies:   string(depsBytes),
		CreatedAt:      now,
		UpdatedAt:      now,
	}

	query := `
		INSERT INTO shared_tasks_v2 (
			id, organization_id, title, description, status, priority, payload, parent_plan_id, dependencies, created_at, updated_at
		) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
	`

	if !s.db.IsSQLite() {
		// PostgreSQL Uses standard placeholders and requires RETURNING or similar handling, but we just need EXEC here
		// Convert ? to $1, $2, etc for PG
		query = `
			INSERT INTO shared_tasks_v2 (
				id, organization_id, title, description, status, priority, payload, parent_plan_id, dependencies, created_at, updated_at
			) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
		`
	}

	_, err := s.db.Exec(ctx, query,
		task.ID, task.OrganizationID, task.Title, task.Description, task.Status, task.Priority, task.Payload, task.ParentPlanID, task.Dependencies, task.CreatedAt, task.UpdatedAt,
	)

	if err != nil {
		return nil, fmt.Errorf("failed to insert task: %w", err)
	}

	broadcastMeshEvent("TASK_DECOMPOSED", task.ID, task.Priority, "")

	return []Task{task}, nil
}

func (s *defaultTaskStore) ClaimNextTask(ctx context.Context, agentID string) (*Task, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()

	// Find a pending task
	query := `
		SELECT id, organization_id, title, description, status, priority, payload, parent_plan_id, dependencies, created_at, updated_at
		FROM shared_tasks_v2
		WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < ?)
		ORDER BY created_at ASC
		LIMIT 1
	`

	if !s.db.IsSQLite() {
		query = `
			SELECT id, organization_id, title, description, status, priority, payload, parent_plan_id, dependencies, created_at, updated_at
			FROM shared_tasks_v2
			WHERE status = 'PENDING' AND (locked_until IS NULL OR locked_until < $1)
			ORDER BY created_at ASC
			LIMIT 1
		`
	}

	row := tx.QueryRow(ctx, query, now)

	var task Task

	// payload could be null
	var payload sql.NullString
	var parentPlanID sql.NullString
	var description sql.NullString

	err = row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &description, &task.Status, &task.Priority, &payload, &parentPlanID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt,
	)

	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to scan task: %w", err)
	}

	if payload.Valid {
		task.Payload = payload.String
	}
	if parentPlanID.Valid {
		task.ParentPlanID = parentPlanID.String
	}
	if description.Valid {
		task.Description = description.String
	}

	// Update task as claimed
	lockedUntilTime := now.Add(5 * time.Minute)
	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID
	task.LockedUntil = lockedUntilTime
	task.UpdatedAt = now

	updateQuery := `
		UPDATE shared_tasks_v2
		SET status = ?, assigned_agent_id = ?, locked_until = ?, updated_at = ?
		WHERE id = ? AND status = 'PENDING'
	`

	if !s.db.IsSQLite() {
		updateQuery = `
			UPDATE shared_tasks_v2
			SET status = $1, assigned_agent_id = $2, locked_until = $3, updated_at = $4
			WHERE id = $5 AND status = 'PENDING'
		`
	}

	rowsAffected, err := tx.Exec(ctx, updateQuery, task.Status, task.AssignedAgentID, task.LockedUntil, task.UpdatedAt, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if rowsAffected == 0 {
		// Task was likely claimed by another worker concurrently
		return nil, nil
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit tx: %w", err)
	}

	broadcastMeshEvent("TASK_CLAIMED", task.ID, task.Priority, agentID)

	return &task, nil
}
