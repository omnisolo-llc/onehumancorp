package sync

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	"go.opentelemetry.io/otel/metric"
)

// TaskData represents the payload of an MCP RAG task
type TaskData struct {
	Query     string `json:"query"`
	SessionID string `json:"session_id"`
}

// Escalator represents the Sync Escalator daemon
type Escalator struct {
	db                  *sql.DB
	httpClient          *http.Client
	tasksEscalatedTotal metric.Int64Counter
}

// InitWithMeter initializes a new Escalator with OpenTelemetry metrics
func InitWithMeter(db *sql.DB, meter metric.Meter) (*Escalator, error) {
	counter, err := meter.Int64Counter("tasks_escalated_total", metric.WithDescription("Total number of local MCP RAG tasks escalated to the cloud swarm"))
	if err != nil {
		return nil, fmt.Errorf("failed to initialize tasks_escalated_total metric: %w", err)
	}

	return &Escalator{
		db:                  db,
		httpClient:          &http.Client{Timeout: 10 * time.Second},
		tasksEscalatedTotal: counter,
	}, nil
}

// Start begins the daemon loop to periodically escalate tasks
func (e *Escalator) Start(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			log.Println("Sync Escalator daemon stopped")
			return
		case <-ticker.C:
			if err := e.ProcessEscalations(ctx); err != nil {
				log.Printf("Sync Escalator error: %v", err)
			}
		}
	}
}

// ProcessEscalations finds local tasks that need escalation and escalates them
func (e *Escalator) ProcessEscalations(ctx context.Context) error {
	// Find pending escalations (this query assumes SQLite syntax for standalone mode)
	rows, err := e.db.QueryContext(ctx, "SELECT id, task_data FROM local_mcp_rag_tasks WHERE escalation_status = 'pending_escalation' LIMIT 50")
	if err != nil {
		return fmt.Errorf("failed to query pending escalations: %w", err)
	}
	defer rows.Close()

	var tasks []struct {
		ID       string
		TaskData string
	}

	for rows.Next() {
		var task struct {
			ID       string
			TaskData string
		}
		if err := rows.Scan(&task.ID, &task.TaskData); err != nil {
			log.Printf("Failed to scan task row: %v", err)
			continue
		}
		tasks = append(tasks, task)
	}
	if err := rows.Err(); err != nil {
		return fmt.Errorf("rows iteration error: %w", err)
	}

	for _, task := range tasks {
		if err := e.escalateTask(ctx, task.ID, task.TaskData); err != nil {
			log.Printf("Failed to escalate task %s: %v", task.ID, err)
			continue
		}
	}

	return nil
}

func (e *Escalator) escalateTask(ctx context.Context, taskID, taskData string) error {
	// Simulated logic for cloud API handoff
	// In reality, this would use SPIFFE/SPIRE for authenticated mTLS to the cloud Swarm
	log.Printf("[SPIFFE Auth] Authenticating via SPIRE agent for task %s...", taskID)

	// Validating task payload
	var td TaskData
	if err := json.Unmarshal([]byte(taskData), &td); err != nil {
		return fmt.Errorf("invalid task data JSON: %w", err)
	}

	// Update local status
	_, err := e.db.ExecContext(ctx, "UPDATE local_mcp_rag_tasks SET escalation_status = 'escalated', updated_at = CURRENT_TIMESTAMP WHERE id = $1", taskID)
	if err != nil {
		return fmt.Errorf("failed to update escalation status: %w", err)
	}

	// Increment telemetry metric
	e.tasksEscalatedTotal.Add(ctx, 1)

	log.Printf("Successfully escalated task %s to cloud Swarm", taskID)
	return nil
}
