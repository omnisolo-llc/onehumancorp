package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"time"

	"github.com/redis/go-redis/v9"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type MeshPayload struct {
	AgentID   string          `json:"agent_id"`
	Action    string          `json:"action"`
	Status    string          `json:"status"`
	Channel   string          `json:"channel,omitempty"`
	EventType string          `json:"event_type,omitempty"`
	Data      json.RawMessage `json:"data,omitempty"`
}

type AutoDreamMemory struct {
	ID             string
	OrganizationID string
	AgentID        string
	TaskID         string
	Content        string
	Embedding      string
	SourceType     string
}

type TeammateMesh struct {
	rdb             *redis.Client
	meter           metric.Meter
	publishLatency  metric.Float64Histogram
	queueLength     metric.Int64UpDownCounter
	autodreamClient *AutoDreamClient
}

type AutoDreamClient struct {
	db       *sql.DB
	isSQLite bool
}

func (a *AutoDreamClient) SummarizeTask(ctx context.Context, payload MeshPayload) error {
	content := string(payload.Data)
	if content == "" {
		content = "Empty payload"
	}

	memory := AutoDreamMemory{
		ID:             payload.AgentID + "_" + payload.Action,
		OrganizationID: "default",
		AgentID:        payload.AgentID,
		TaskID:         "task_sim",
		Content:        content,
		Embedding:      "[0.1, 0.2, 0.3]", // Simulated vector, real app would call LLM here
		SourceType:     "TASK_SUMMARY",
	}

    if a.db == nil {
        return nil
    }

	if a.isSQLite {
        _, err := a.db.ExecContext(ctx, "INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6, $7)", memory.ID, memory.OrganizationID, memory.AgentID, memory.TaskID, memory.Content, memory.Embedding, memory.SourceType)
        return err
	} else {
        _, err := a.db.ExecContext(ctx, "INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)", memory.ID, memory.OrganizationID, memory.AgentID, memory.TaskID, memory.Content, memory.Embedding, memory.SourceType)
        return err
	}
}

func NewTeammateMesh(redisAddr string, db *sql.DB, isSQLite bool) *TeammateMesh {
	rdb := redis.NewClient(&redis.Options{Addr: redisAddr})
	meter := otel.GetMeterProvider().Meter("ohc.orchestration.mesh")
	publishLatency, _ := meter.Float64Histogram("mesh.tasks.publish_latency")
	queueLength, _ := meter.Int64UpDownCounter("mesh.tasks.queue_length")

	return &TeammateMesh{
		rdb:             rdb,
		meter:           meter,
		publishLatency:  publishLatency,
		queueLength:     queueLength,
		autodreamClient: &AutoDreamClient{db: db, isSQLite: isSQLite},
	}
}

func (m *TeammateMesh) PublishTaskStatus(ctx context.Context, agentID, action, status string, data json.RawMessage) error {
	start := time.Now()
	payload := MeshPayload{
		AgentID:   agentID,
		Action:    action,
		Status:    status,
		Channel:   "mesh:tasks",
		EventType: "TASK",
		Data:      data,
	}
	bytes, err := json.Marshal(payload)
	if err != nil {
		return err
	}

	err = m.rdb.Publish(ctx, "mesh:tasks", bytes).Err()
	m.publishLatency.Record(ctx, time.Since(start).Seconds())
	m.queueLength.Add(ctx, 1) // Increment queue for visibility

	// AutoDream summarization trigger
	if status == "COMPLETED" || status == "FAILED" {
		go func() {
            err := m.autodreamClient.SummarizeTask(context.Background(), payload)
            if err != nil {
                log.Printf("Failed to summarize task: %v", err)
            }
        }()
	}

	return err
}

func (m *TeammateMesh) SubscribeToTasks(ctx context.Context, handler func(MeshPayload)) {
	sub := m.rdb.Subscribe(ctx, "mesh:tasks")
	ch := sub.Channel()
	go func() {
		for msg := range ch {
			var payload MeshPayload
			if err := json.Unmarshal([]byte(msg.Payload), &payload); err != nil {
				log.Printf("Failed to unmarshal mesh payload: %v", err)
				continue
			}
			m.queueLength.Add(ctx, -1) // Decrement queue length as task is processed
			handler(payload)
		}
	}()
}
