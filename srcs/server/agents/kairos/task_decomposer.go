package kairos

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

var (
	ErrCircularDependency = errors.New("circular dependency detected in tasks")
	ErrTaskNotFound       = errors.New("task not found")
	ErrLockFailed         = errors.New("failed to acquire lock")
)

// Task represents a task in the decomposition graph.
type Task struct {
	ID              string
	OrganizationID  string
	Title           string
	Description     string
	Status          string
	AssignedAgentID string
	Priority        string
	Payload         []byte
	ParentPlanID    string
	Dependencies    []string
	LockedUntil     *time.Time
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

// TaskDecomposer manages the task decomposition lifecycle and state machine.
type TaskDecomposer struct {
	provider db.Provider
}

// NewTaskDecomposer creates a new TaskDecomposer.
func NewTaskDecomposer(provider db.Provider) *TaskDecomposer {
	return &TaskDecomposer{
		provider: provider,
	}
}

// CreateTasks inserts multiple tasks and ensures no circular dependencies exist among them.
func (td *TaskDecomposer) CreateTasks(ctx context.Context, tasks []*Task) error {
	if err := td.checkCircularDependencies(ctx, tasks); err != nil {
		return err
	}

	tx, err := td.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, t := range tasks {
		if t.ID == "" {
			t.ID = uuid.NewString()
		}
		if t.Status == "" {
			t.Status = "PENDING"
		}
		if t.Priority == "" {
			t.Priority = "P2"
		}

		depsJSON, err := json.Marshal(t.Dependencies)
		if err != nil {
			return fmt.Errorf("failed to marshal dependencies: %w", err)
		}

		query := `
			INSERT INTO shared_tasks_decomposition (
				id, organization_id, title, description, status, assigned_agent_id,
				priority, payload, parent_plan_id, dependencies
			) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
		`
		var payloadStr interface{}
		if t.Payload != nil {
			if td.provider.IsSQLite() {
				payloadStr = string(t.Payload)
			} else {
				payloadStr = t.Payload
			}
		}

		_, err = tx.Exec(ctx, query, t.ID, t.OrganizationID, t.Title, t.Description,
			t.Status, t.AssignedAgentID, t.Priority, payloadStr, t.ParentPlanID, string(depsJSON))
		if err != nil {
			return fmt.Errorf("failed to insert task %s: %w", t.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

// UpdateTaskStatus updates a task's status.
func (td *TaskDecomposer) UpdateTaskStatus(ctx context.Context, taskID string, status string) error {
	query := `UPDATE shared_tasks_decomposition SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	result, err := td.provider.Exec(ctx, query, status, taskID)
	if err != nil {
		return fmt.Errorf("failed to update task status: %w", err)
	}
	if result == 0 {
		return ErrTaskNotFound
	}
	return nil
}

// AcquirePendingTask attempts to lock and return a pending task whose dependencies are satisfied.
func (td *TaskDecomposer) AcquirePendingTask(ctx context.Context, organizationID, agentID string) (*Task, error) {
	tx, err := td.provider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var lockedTask Task
	var depsJSON string
	var payloadStr *string

	if td.provider.IsSQLite() {
		query := `SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies FROM shared_tasks_decomposition WHERE organization_id = $1 AND status != 'COMPLETED'`
		rows, err := tx.Query(ctx, query, organizationID)
		if err != nil {
			return nil, fmt.Errorf("failed to query tasks: %w", err)
		}

		var allTasks []*Task
		taskMap := make(map[string]*Task)
		for rows.Next() {
			var t Task
			var dJSON string
			var pStr *string
			var assignID *string
			if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Status, &assignID, &t.Priority, &pStr, &t.ParentPlanID, &dJSON); err != nil {
				rows.Close()
				return nil, fmt.Errorf("failed to scan task: %w", err)
			}
			if assignID != nil {
				t.AssignedAgentID = *assignID
			}
			if pStr != nil {
				t.Payload = []byte(*pStr)
			}
			if err := json.Unmarshal([]byte(dJSON), &t.Dependencies); err != nil {
				rows.Close()
				return nil, fmt.Errorf("failed to unmarshal dependencies: %w", err)
			}
			allTasks = append(allTasks, &t)
			taskMap[t.ID] = &t
		}
		rows.Close()

		var targetTaskID string
		for _, t := range allTasks {
			if t.Status == "PENDING" && (t.AssignedAgentID == "" || t.AssignedAgentID == agentID) {
				depsSatisfied := true
				for _, depID := range t.Dependencies {
					if dep, ok := taskMap[depID]; !ok || dep.Status != "COMPLETED" {
						if !ok {
							var depStatus string
							err := tx.QueryRow(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = $1", depID).Scan(&depStatus)
							if err != nil || depStatus != "COMPLETED" {
								depsSatisfied = false
								break
							}
						} else {
							depsSatisfied = false
							break
						}
					}
				}
				if depsSatisfied {
					targetTaskID = t.ID
					break
				}
			}
		}

		if targetTaskID == "" {
			return nil, nil // No task available
		}

		updateQuery := `
			UPDATE shared_tasks_decomposition
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = datetime('now', '+5 minutes'), updated_at = CURRENT_TIMESTAMP
			WHERE id = $2 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < datetime('now'))
		`
		res, err := tx.Exec(ctx, updateQuery, agentID, targetTaskID)
		if err != nil {
			return nil, fmt.Errorf("failed to update sqlite lock: %w", err)
		}
		if res == 0 {
			return nil, ErrLockFailed
		}

		var assignID *string
		selectQuery := `SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies FROM shared_tasks_decomposition WHERE id = $1`
		err = tx.QueryRow(ctx, selectQuery, targetTaskID).Scan(
			&lockedTask.ID, &lockedTask.OrganizationID, &lockedTask.Title, &lockedTask.Description,
			&lockedTask.Status, &assignID, &lockedTask.Priority, &payloadStr,
			&lockedTask.ParentPlanID, &depsJSON,
		)
		if err != nil {
			return nil, fmt.Errorf("failed to read back locked task: %w", err)
		}
		if assignID != nil {
			lockedTask.AssignedAgentID = *assignID
		}

	} else {
		selectQuery := `
			SELECT id, organization_id, title, description, status, assigned_agent_id, priority, payload, parent_plan_id, dependencies
			FROM shared_tasks_decomposition
			WHERE organization_id = $1 AND status = 'PENDING' AND (assigned_agent_id IS NULL OR assigned_agent_id = '' OR assigned_agent_id = $2)
			FOR UPDATE SKIP LOCKED LIMIT 10
		`

		rows, err := tx.Query(ctx, selectQuery, organizationID, agentID)
		if err != nil {
			return nil, fmt.Errorf("failed to query pg pending tasks: %w", err)
		}

		var pendingTasks []*Task
		for rows.Next() {
			var t Task
			var dJSON string
			var pStr *string
			var assignID *string
			if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Status, &assignID, &t.Priority, &pStr, &t.ParentPlanID, &dJSON); err != nil {
				rows.Close()
				return nil, fmt.Errorf("failed to scan pg task: %w", err)
			}
			if assignID != nil {
				t.AssignedAgentID = *assignID
			}
			if pStr != nil {
				t.Payload = []byte(*pStr)
			}
			if err := json.Unmarshal([]byte(dJSON), &t.Dependencies); err != nil {
				rows.Close()
				return nil, fmt.Errorf("failed to unmarshal pg dependencies: %w", err)
			}
			pendingTasks = append(pendingTasks, &t)
		}
		rows.Close()

		var targetTaskID string
		for _, t := range pendingTasks {
			if len(t.Dependencies) == 0 {
				targetTaskID = t.ID
				break
			}

			depsSatisfied := true
			for _, depID := range t.Dependencies {
				var depStatus string
				err := tx.QueryRow(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = $1", depID).Scan(&depStatus)
				if err != nil || depStatus != "COMPLETED" {
					depsSatisfied = false
					break
				}
			}
			if depsSatisfied {
				targetTaskID = t.ID
				break
			}
		}

		if targetTaskID == "" {
			return nil, nil
		}

		updateQuery := `UPDATE shared_tasks_decomposition SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, targetTaskID)
		if err != nil {
			return nil, fmt.Errorf("failed to update pg lock: %w", err)
		}

		for _, t := range pendingTasks {
			if t.ID == targetTaskID {
				lockedTask = *t
				lockedTask.Status = "IN_PROGRESS"
				lockedTask.AssignedAgentID = agentID

				depsBytes, _ := json.Marshal(t.Dependencies)
				depsJSON = string(depsBytes)

				if t.Payload != nil {
					pStr := string(t.Payload)
					payloadStr = &pStr
				}
				break
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit lock tx: %w", err)
	}

	if payloadStr != nil {
		lockedTask.Payload = []byte(*payloadStr)
	}
	if err := json.Unmarshal([]byte(depsJSON), &lockedTask.Dependencies); err != nil {
		return nil, fmt.Errorf("failed to unmarshal deps on locked task: %w", err)
	}

	return &lockedTask, nil
}

// checkCircularDependencies performs a topological sort to detect cycles.
// It checks the new tasks and their existing dependencies in the database.
func (td *TaskDecomposer) checkCircularDependencies(ctx context.Context, tasks []*Task) error {
	inDegree := make(map[string]int)
	graph := make(map[string][]string)
	nodes := make(map[string]struct{})

	// Gather all dependencies to query their dependencies from DB
	var depIDs []string

	for _, t := range tasks {
		nodes[t.ID] = struct{}{}
		inDegree[t.ID] = 0
		for _, dep := range t.Dependencies {
			depIDs = append(depIDs, dep)
		}
	}

	// For simplicity, we just fetch all tasks for the org.
	if len(tasks) > 0 {
		orgID := tasks[0].OrganizationID
		query := `SELECT id, dependencies FROM shared_tasks_decomposition WHERE organization_id = $1`
		rows, err := td.provider.Query(ctx, query, orgID)
		if err == nil {
			defer rows.Close()
			for rows.Next() {
				var dbID string
				var dbDepsJSON string
				if err := rows.Scan(&dbID, &dbDepsJSON); err == nil {
					var dbDeps []string
					if err := json.Unmarshal([]byte(dbDepsJSON), &dbDeps); err == nil {
						if _, exists := nodes[dbID]; !exists {
							nodes[dbID] = struct{}{}
							inDegree[dbID] = 0
						}
						for _, dep := range dbDeps {
							graph[dep] = append(graph[dep], dbID)
							inDegree[dbID]++
							if _, exists := nodes[dep]; !exists {
								nodes[dep] = struct{}{}
								inDegree[dep] = 0
							}
						}
					}
				}
			}
		}
	}

	// Add new tasks to graph
	for _, t := range tasks {
		for _, dep := range t.Dependencies {
			graph[dep] = append(graph[dep], t.ID)
			inDegree[t.ID]++
			if _, exists := nodes[dep]; !exists {
				nodes[dep] = struct{}{}
				inDegree[dep] = 0
			}
		}
	}

	var queue []string
	for node, degree := range inDegree {
		if degree == 0 {
			queue = append(queue, node)
		}
	}

	visitedCount := 0
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		visitedCount++

		for _, v := range graph[u] {
			inDegree[v]--
			if inDegree[v] == 0 {
				queue = append(queue, v)
			}
		}
	}

	if visitedCount != len(nodes) {
		return ErrCircularDependency
	}

	return nil
}
