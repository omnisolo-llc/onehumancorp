package main

import (
	"os"
	"strings"
)

func main() {
	// mesh.go
	content, err := os.ReadFile("srcs/server/orchestration/mesh.go")
	if err == nil {
		newType := `
// V2TeammateMesh implements TeammateMesh utilizing CentrifugeNode for resilient,
// real-time pub/sub instead of bare websockets.
type V2TeammateMesh struct {
	node *CentrifugeNode
}

func NewV2TeammateMesh(node *CentrifugeNode) *V2TeammateMesh {
	return &V2TeammateMesh{node: node}
}

func (vm *V2TeammateMesh) BroadcastTask(ctx context.Context, task Task) error {
	payload := map[string]interface{}{
		"agent_id": task.AgentID,
		"action":   task.Action,
		"status":   task.Status,
		"task_id":  task.TaskID,
	}
	vm.node.PublishTaskBroadcast(task.TaskID, payload)
	return nil
}

func (vm *V2TeammateMesh) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	ch := make(chan Task)
	return ch, nil
}

func (vm *V2TeammateMesh) BroadcastCoordination(ctx context.Context, msg MeshMessage) error {
	payload := Message{
		ID:        msg.AgentID,
		AgentID:   msg.AgentID,
		Content:   msg.Content,
		Role:      msg.Role,
		Timestamp: msg.Timestamp,
	}
	vm.node.PublishCoordinationMessage(payload)
	return nil
}

func (vm *V2TeammateMesh) SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error) {
	ch := make(chan MeshMessage)
	return ch, nil
}
`
		if !strings.Contains(string(content), "V2TeammateMesh") {
			newContent := strings.Replace(string(content), "type TeammateMesh interface {\n\tBroadcastTask(ctx context.Context, task Task) error\n\tSubscribeTasks(ctx context.Context) (<-chan Task, error)\n\tBroadcastCoordination(ctx context.Context, msg MeshMessage) error\n\tSubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error)\n}", "type TeammateMesh interface {\n\tBroadcastTask(ctx context.Context, task Task) error\n\tSubscribeTasks(ctx context.Context) (<-chan Task, error)\n\tBroadcastCoordination(ctx context.Context, msg MeshMessage) error\n\tSubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error)\n}\n"+newType, 1)
			os.WriteFile("srcs/server/orchestration/mesh.go", []byte(newContent), 0644)
		}
	}

	// queue.go
	contentQueue, err := os.ReadFile("srcs/server/orchestration/queue.go")
	if err == nil {
		if !strings.Contains(string(contentQueue), "SubAgentQueue") {
			newInterface := `

// SubAgentQueue defines a distributed sub-agent queue interface.
type SubAgentQueue interface {
	Enqueue(ctx context.Context, parentTaskID string, payload map[string]interface{}) (string, error)
	Dequeue(ctx context.Context) (*QueuedTask, error)
	Complete(ctx context.Context, taskID string) error
}

// PgRedisQueue implements SubAgentQueue for Cloud-Native Mode using Rueidis ZSETs.
type PgRedisQueue struct {
	client rueidis.Client
}

func NewPgRedisQueue(client rueidis.Client) *PgRedisQueue {
	return &PgRedisQueue{client: client}
}

func (q *PgRedisQueue) Enqueue(ctx context.Context, parentTaskID string, payload map[string]interface{}) (string, error) {
	id := generateID()
	jobData := map[string]interface{}{
		"id": id,
		"parent_task_id": parentTaskID,
		"payload": payload,
	}
	jobBytes, err := json.Marshal(jobData)
	if err != nil {
		return "", err
	}

	score := float64(time.Now().UnixMilli())
	cmd := q.client.B().Zadd().Key("sub_agent_queue").ScoreMember().ScoreMember(score, string(jobBytes)).Build()
	if err := q.client.Do(ctx, cmd).Error(); err != nil {
		return "", fmt.Errorf("failed to enqueue to redis: %w", err)
	}
	return id, nil
}

func (q *PgRedisQueue) Dequeue(ctx context.Context) (*QueuedTask, error) {
	cmd := q.client.B().Zpopmin().Key("sub_agent_queue").Count(1).Build()
	resp := q.client.Do(ctx, cmd)
	if err := resp.Error(); err != nil {
		if rueidis.IsRedisNil(err) {
			return nil, nil
		}
		return nil, err
	}
	items, err := resp.AsZScores()
	if err != nil || len(items) == 0 {
		return nil, err
	}
	item := items[0].Member

	var jobData struct {
		ID      string                 ` + "`" + `json:"id"` + "`" + `
		Payload map[string]interface{} ` + "`" + `json:"payload"` + "`" + `
	}
	if err := json.Unmarshal([]byte(item), &jobData); err != nil {
		return nil, fmt.Errorf("failed to unmarshal job: %w", err)
	}

	return &QueuedTask{
		ID:      jobData.ID,
		Payload: jobData.Payload,
	}, nil
}

func (q *PgRedisQueue) Complete(ctx context.Context, taskID string) error {
	return nil
}

// SqliteQueue implements SubAgentQueue for Standalone Mode using SQLite.
type SqliteQueue struct {
	db db.Provider
	mu sync.Mutex
}

func NewSqliteQueue(db db.Provider) *SqliteQueue {
	return &SqliteQueue{db: db}
}

func (q *SqliteQueue) ensureTable(ctx context.Context) error {
	q.mu.Lock()
	defer q.mu.Unlock()
	query := ` + "`" + `
		CREATE TABLE IF NOT EXISTS sub_agent_queue (
			id TEXT PRIMARY KEY,
			parent_task_id TEXT NOT NULL,
			payload TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			scheduled_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			completed_at DATETIME
		)
	` + "`" + `
	_, err := q.db.Exec(ctx, query)
	return err
}

func (q *SqliteQueue) Enqueue(ctx context.Context, parentTaskID string, payload map[string]interface{}) (string, error) {
	if err := q.ensureTable(ctx); err != nil {
		return "", err
	}

	id := generateID()
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}

	q.mu.Lock()
	defer q.mu.Unlock()

	query := ` + "`" + `
		INSERT INTO sub_agent_queue (id, parent_task_id, payload, status)
		VALUES ($1, $2, $3, 'PENDING')
	` + "`" + `
	_, err = q.db.Exec(ctx, query, id, parentTaskID, string(payloadBytes))
	return id, err
}

func (q *SqliteQueue) Dequeue(ctx context.Context) (*QueuedTask, error) {
	if err := q.ensureTable(ctx); err != nil {
		return nil, err
	}

	q.mu.Lock()
	defer q.mu.Unlock()

	var id, payloadStr string
	query := ` + "`" + `
		UPDATE sub_agent_queue
		SET status = 'PROCESSING'
		WHERE id IN (
			SELECT id FROM sub_agent_queue
			WHERE status = 'PENDING' AND scheduled_at <= CURRENT_TIMESTAMP
			ORDER BY scheduled_at ASC LIMIT 1
		)
		RETURNING id, payload
	` + "`" + `
	err := q.db.QueryRow(ctx, query).Scan(&id, &payloadStr)
	if err != nil {
		return nil, nil // Assume sql.ErrNoRows or empty
	}

	var payload map[string]interface{}
	if err := json.Unmarshal([]byte(payloadStr), &payload); err != nil {
		return nil, err
	}

	return &QueuedTask{
		ID:      id,
		Payload: payload,
	}, nil
}

func (q *SqliteQueue) Complete(ctx context.Context, taskID string) error {
	q.mu.Lock()
	defer q.mu.Unlock()
	_, err := q.db.Exec(ctx, "UPDATE sub_agent_queue SET status = 'COMPLETED', completed_at = CURRENT_TIMESTAMP WHERE id = $1", taskID)
	return err
}
`
			newContent := strings.Replace(string(contentQueue), `import (`, "import (\n\t\"sync\"", 1)
			newContent += newInterface
			os.WriteFile("srcs/server/orchestration/queue.go", []byte(newContent), 0644)
		}
	}

	// tasks.go
	contentTasks, err := os.ReadFile("srcs/server/orchestration/tasks.go")
	if err == nil {
		oldCode := `	for _, task := range claimedTasks {
		// Broadcast task claim
		if tm.hub != nil {
			go func(t *SharedTask) {
				payload := map[string]interface{}{
					"task_id":  t.ID,
					"action":   "CLAIM",
					"agent_id": agentID,
					"status":   t.Status,
				}
				tm.hub.PublishTaskBroadcast(t.ID, payload)
			}(task)
		}
	}`

		newCode := `	for _, task := range claimedTasks {
		// Broadcast task claim using State Machine tracking (V2)
		if tm.hub != nil {
			go func(t *SharedTask) {
				msg := MeshMessage{
					AgentID:   agentID,
					Action:    "CLAIM",
					Status:    t.Status,
					Timestamp: time.Now(),
					Content:   t.ID,
				}

				payload := map[string]interface{}{
					"agent_id": msg.AgentID,
					"action":   msg.Action,
					"status":   msg.Status,
					"task_id":  t.ID,
					"timestamp": msg.Timestamp,
				}
				tm.hub.PublishTaskBroadcast(t.ID, payload)
			}(task)
		}
	}`
		if !strings.Contains(string(contentTasks), "MeshMessage tracking") {
			newContent := strings.Replace(string(contentTasks), oldCode, newCode, 1)
			os.WriteFile("srcs/server/orchestration/tasks.go", []byte(newContent), 0644)
		}
	}
}
