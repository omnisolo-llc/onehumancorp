package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"sync"
	"time"
)

// defaultHeartbeatTimeout is the maximum time the parent waits for the sub-agent
// to finish before forcefully marking it as killed.
// Override via OHC_SUBAGENT_HEARTBEAT_TIMEOUT (e.g. "10m").
const defaultHeartbeatTimeout = 30 * time.Minute

// agentRegistry is a process-level registry for spawned sub-agents.
// It maps taskID → *SubagentState for progress querying and kill operations.
var agentRegistry = &SubagentRegistry{tasks: make(map[string]*SubagentState)}

// SubagentState tracks a spawned sub-agent.
type SubagentState struct {
	TaskID      string
	Description string
	Status      string // "running", "completed", "failed", "killed"
	Result      string
	Error       string
	StartedAt   time.Time
	EndedAt     time.Time
	ToolUseID   string
	OutputFile  string
	TokenCount  int64
	ToolUses    int64
}

// SubagentRegistry is a thread-safe map of spawned sub-agents.
type SubagentRegistry struct {
	mu    sync.RWMutex
	tasks map[string]*SubagentState
}

// Register adds or replaces a sub-agent state entry.
func (r *SubagentRegistry) Register(s *SubagentState) {
	r.mu.Lock()
	r.tasks[s.TaskID] = s
	r.mu.Unlock()
}

// Get returns a state entry.
func (r *SubagentRegistry) Get(taskID string) (*SubagentState, bool) {
	r.mu.RLock()
	s, ok := r.tasks[taskID]
	r.mu.RUnlock()
	return s, ok
}

// Kill cancels a running sub-agent.
func (r *SubagentRegistry) Kill(taskID string) bool {
	r.mu.RLock()
	s, ok := r.tasks[taskID]
	r.mu.RUnlock()
	if !ok || s.Status != "running" {
		return false
	}
	s.Status = "killed"
	s.EndedAt = time.Now()
	return true
}

// All returns a snapshot of all entries.
func (r *SubagentRegistry) All() []*SubagentState {
	r.mu.RLock()
	defer r.mu.RUnlock()
	out := make([]*SubagentState, 0, len(r.tasks))
	for _, s := range r.tasks {
		cp := *s
		out = append(out, &cp)
	}
	return out
}

// heartbeatTimeout returns the configured heartbeat timeout.
func heartbeatTimeout() time.Duration {
	if s := os.Getenv("OHC_SUBAGENT_HEARTBEAT_TIMEOUT"); s != "" {
		if seconds, err := strconv.Atoi(s); err == nil && seconds > 0 {
			return time.Duration(seconds) * time.Second
		}
		if d, err := time.ParseDuration(s); err == nil && d > 0 {
			return d
		}
	}
	return defaultHeartbeatTimeout
}

// AgentTool spawns a background sub-agent task and returns immediately.
// Mirrors CC-Source's AgentTool which spawns a LocalAgentTask.
// The spawned task runs using the builtin SpawnTask path.
//
// The parent agent is notified via the SubagentBus when the child completes.
// If no completion signal is received within the heartbeat timeout, the task
// is marked as killed (no polling required).
var AgentTool = Tool{
	Name: "Agent",
	Description: "Spawn a background agent to perform a task concurrently. " +
		"Returns immediately with a task ID. The agent runs asynchronously " +
		"and you will receive a SubagentLifecycleEvent via the SubagentBus " +
		"when it completes. Use this to delegate work or run tasks in parallel.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"description": {
				"type": "string",
				"description": "A short (3-5 word) description of the task"
			},
			"prompt": {
				"type": "string",
				"description": "The task for the sub-agent to perform"
			},
			"subagent_type": {
				"type": "string",
				"description": "Optional agent type (e.g., 'general-purpose'). Defaults to 'general-purpose'."
			}
		},
		"required": ["description", "prompt"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Description  string `json:"description"`
			Prompt       string `json:"prompt"`
			SubagentType string `json:"subagent_type"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", fmt.Errorf("Agent: invalid args: %w", err)
		}
		if input.Description == "" {
			return "", fmt.Errorf("Agent: description is required")
		}
		if input.Prompt == "" {
			return "", fmt.Errorf("Agent: prompt is required")
		}

		// Compress the prompt for agent-to-agent communication (caveman full mode).
		// This saves ~75% of output tokens while keeping full technical accuracy.
		compressedPrompt := CavemanCompress(input.Prompt, CavemanFull)
		agentCfg := AgentConfig{
			SystemPromptSuffix: cavemanSystemPrompt(CavemanFull),
		}

		// Spawn the sub-task using the local runtime.
		taskState, err := SpawnTask(ctx, input.Description, compressedPrompt, "", agentCfg)
		if err != nil {
			return "", fmt.Errorf("Agent: spawn failed: %w", err)
		}

		// Register in the global registry so it can be stopped/queried.
		entry := &SubagentState{
			TaskID:      taskState.ID,
			Description: input.Description,
			Status:      "running",
			StartedAt:   time.Now(),
			OutputFile:  taskState.OutputFile,
		}
		agentRegistry.Register(entry)

		// Publish a "spawned" lifecycle event on the bus.
		globalSubagentBus.Publish(SubagentLifecycleEvent{
			EventType: SubagentEventSpawned,
			TaskID:    taskState.ID,
		})

		// Watch for completion using Done() channel + heartbeat timeout.
		// This avoids polling — the goroutine blocks until the task signals
		// completion or the heartbeat deadline expires.
		timeout := heartbeatTimeout()
		go func() {
			select {
			case <-taskState.Done():
				// Task entered a terminal state normally.
			case <-time.After(timeout):
				// Heartbeat timeout — force-kill the task.
				taskState.Kill()
			}

			st := taskState.Status()
			entry.Status = string(st)
			entry.EndedAt = time.Now()
			entry.Result = taskState.Result()
			entry.Error = taskState.Err()
			p := taskState.Progress()
			entry.TokenCount = p.TokenCount
			entry.ToolUses = int64(p.ToolUseCount)
			agentRegistry.Register(entry)

			// Build and publish the typed TaskNotification.
			summary := fmt.Sprintf("Sub-agent %q finished with status %s", input.Description, entry.Status)
			notif := BuildTaskNotificationMsg(
				entry.TaskID, entry.ToolUseID, entry.OutputFile,
				entry.Status, summary, entry.Result,
				entry.TokenCount, entry.ToolUses,
				entry.EndedAt.Sub(entry.StartedAt),
			)

			// Determine event type from final status.
			evtType := SubagentEventCompleted
			switch entry.Status {
			case "failed":
				evtType = SubagentEventFailed
			case "killed":
				evtType = SubagentEventKilled
			}

			globalSubagentBus.Publish(SubagentLifecycleEvent{
				EventType:    evtType,
				TaskID:       entry.TaskID,
				Notification: notif,
			})
		}()

		out := map[string]interface{}{
			"status":      "async_launched",
			"agentId":     taskState.ID,
			"description": input.Description,
			"outputFile":  taskState.OutputFile,
		}
		b, _ := json.Marshal(out)
		return string(b), nil
	},
}

// TaskStopTool stops a running sub-agent by its task ID.
// Mirrors CC-Source's TaskStopTool.
var TaskStopTool = Tool{
	Name: "TaskStop",
	Description: "Stop a running background agent task. " +
		"Use the task ID returned by the Agent tool.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"task_id": {
				"type": "string",
				"description": "The task ID of the agent to stop"
			}
		},
		"required": ["task_id"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			TaskID string `json:"task_id"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", fmt.Errorf("TaskStop: invalid args: %w", err)
		}
		if input.TaskID == "" {
			return "", fmt.Errorf("TaskStop: task_id is required")
		}
		if agentRegistry.Kill(input.TaskID) {
			globalSubagentBus.Publish(SubagentLifecycleEvent{
				EventType: SubagentEventKilled,
				TaskID:    input.TaskID,
			})
			return fmt.Sprintf("Task %s stopped.", input.TaskID), nil
		}
		return fmt.Sprintf("Task %s not found or already terminal.", input.TaskID), nil
	},
}

// TaskStatusTool returns the status of a running sub-agent task.
var TaskStatusTool = Tool{
	Name:        "TaskStatus",
	Description: "Get the current status and progress of a background agent task.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"task_id": {
				"type": "string",
				"description": "The task ID to query"
			}
		},
		"required": ["task_id"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			TaskID string `json:"task_id"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", fmt.Errorf("TaskStatus: invalid args: %w", err)
		}
		state, ok := agentRegistry.Get(input.TaskID)
		if !ok {
			return fmt.Sprintf("Task %s not found.", input.TaskID), nil
		}
		b, _ := json.Marshal(map[string]interface{}{
			"task_id":     state.TaskID,
			"description": state.Description,
			"status":      state.Status,
			"result":      state.Result,
			"error":       state.Error,
			"token_count": state.TokenCount,
			"tool_uses":   state.ToolUses,
		})
		return string(b), nil
	},
}

