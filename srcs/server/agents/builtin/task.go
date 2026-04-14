package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

// taskStatus constants mirror CC-Source's TaskStatus type.
const (
	taskStatusPending    = "pending"
	taskStatusInProgress = "in_progress"
	taskStatusCompleted  = "completed"
	taskStatusDeleted    = "deleted"
)

// taskEntry represents one task in the thread-safe task store.
type taskEntry struct {
	ID          string            `json:"id"`
	Subject     string            `json:"subject"`
	Description string            `json:"description"`
	Status      string            `json:"status"`
	Owner       string            `json:"owner,omitempty"`
	ActiveForm  string            `json:"activeForm,omitempty"` // e.g. "Running tests"
	Blocks      []string          `json:"blocks"`
	BlockedBy   []string          `json:"blockedBy"`
	Metadata    map[string]any    `json:"metadata,omitempty"`
	CreatedAt   time.Time         `json:"created_at"`
	UpdatedAt   time.Time         `json:"updated_at"`
}

// globalTaskStore is the process-level task store.
// Thread-safe; backed by a sync.Map for lock-free reads on the hot path.
var globalTaskStore = &taskStore{}

type taskStore struct {
	mu      sync.RWMutex
	entries map[string]*taskEntry
	counter atomic.Int64
}

func (s *taskStore) ensureInit() {
	s.mu.Lock()
	if s.entries == nil {
		s.entries = make(map[string]*taskEntry)
	}
	s.mu.Unlock()
}

func (s *taskStore) create(subject, description, activeForm string, metadata map[string]any) *taskEntry {
	s.ensureInit()
	id := fmt.Sprintf("%d", s.counter.Add(1))
	now := time.Now().UTC()
	e := &taskEntry{
		ID:          id,
		Subject:     subject,
		Description: description,
		Status:      taskStatusPending,
		ActiveForm:  activeForm,
		Blocks:      []string{},
		BlockedBy:   []string{},
		Metadata:    metadata,
		CreatedAt:   now,
		UpdatedAt:   now,
	}
	s.mu.Lock()
	s.entries[id] = e
	s.mu.Unlock()
	return e
}

func (s *taskStore) get(id string) (*taskEntry, bool) {
	s.ensureInit()
	s.mu.RLock()
	e, ok := s.entries[id]
	s.mu.RUnlock()
	return e, ok
}

func (s *taskStore) list() []*taskEntry {
	s.ensureInit()
	s.mu.RLock()
	out := make([]*taskEntry, 0, len(s.entries))
	for _, e := range s.entries {
		if e.Status != taskStatusDeleted {
			out = append(out, e)
		}
	}
	s.mu.RUnlock()
	return out
}

func (s *taskStore) update(
	id, subject, description, activeForm, status, owner string,
	addBlocks, addBlockedBy []string,
	metadata map[string]any,
) (*taskEntry, error) {
	s.ensureInit()
	s.mu.Lock()
	defer s.mu.Unlock()
	e, ok := s.entries[id]
	if !ok {
		return nil, fmt.Errorf("task %q not found", id)
	}
	if subject != "" {
		e.Subject = subject
	}
	if description != "" {
		e.Description = description
	}
	if activeForm != "" {
		e.ActiveForm = activeForm
	}
	if status != "" {
		if status == taskStatusDeleted {
			delete(s.entries, id)
			return e, nil
		}
		e.Status = status
	}
	if owner != "" {
		e.Owner = owner
	}
	for _, b := range addBlocks {
		e.Blocks = appendUnique(e.Blocks, b)
	}
	for _, b := range addBlockedBy {
		e.BlockedBy = appendUnique(e.BlockedBy, b)
	}
	for k, v := range metadata {
		if e.Metadata == nil {
			e.Metadata = make(map[string]any)
		}
		if v == nil {
			delete(e.Metadata, k)
		} else {
			e.Metadata[k] = v
		}
	}
	e.UpdatedAt = time.Now().UTC()
	return e, nil
}

func appendUnique(s []string, v string) []string {
	for _, x := range s {
		if x == v {
			return s
		}
	}
	return append(s, v)
}

// ─── Task tools ────────────────────────────────────────────────────────────────

// Task represents a task in the system. (Legacy compatibility alias.)
type Task = taskEntry

// TaskCreateTool definition - matches CC-Source TaskCreateTool.
var TaskCreateTool = Tool{
	Name:        "TaskCreate",
	Description: "Create a new task in the task list.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"subject": {"type": "string", "description": "A brief title for the task"},
			"description": {"type": "string", "description": "What needs to be done"},
			"activeForm": {
				"type": "string",
				"description": "Present continuous form shown in spinner when in_progress (e.g. 'Running tests')"
			},
			"metadata": {
				"type": "object",
				"description": "Arbitrary metadata to attach to the task"
			}
		},
		"required": ["subject", "description"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Subject     string         `json:"subject"`
			Description string         `json:"description"`
			ActiveForm  string         `json:"activeForm"`
			Metadata    map[string]any `json:"metadata"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}
		if input.Subject == "" {
			return "", fmt.Errorf("TaskCreate: subject is required")
		}
		task := globalTaskStore.create(input.Subject, input.Description, input.ActiveForm, input.Metadata)
		b, _ := json.Marshal(map[string]interface{}{
			"task": map[string]interface{}{"id": task.ID, "subject": task.Subject},
		})
		return fmt.Sprintf("Task #%s created successfully: %s\n%s", task.ID, task.Subject, string(b)), nil
	},
}

// TaskGetTool definition - matches CC-Source TaskGetTool.
var TaskGetTool = Tool{
	Name:        "TaskGet",
	Description: "Get details of a specific task by ID.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"taskId": {"type": "string", "description": "The ID of the task to retrieve"}
		},
		"required": ["taskId"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			TaskID string `json:"taskId"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}
		if input.TaskID == "" {
			// backward compat: also try task_id
			var alt struct {
				TaskID string `json:"task_id"`
			}
			_ = json.Unmarshal(args, &alt)
			input.TaskID = alt.TaskID
		}
		task, ok := globalTaskStore.get(input.TaskID)
		if !ok {
			return "Task not found.", nil
		}
		b, _ := json.MarshalIndent(task, "", "  ")
		return string(b), nil
	},
}

// TaskListTool definition - matches CC-Source TaskListTool.
var TaskListTool = Tool{
	Name:        "TaskList",
	Description: "List all tasks in the task list.",
	Parameters:  json.RawMessage(`{"type": "object", "properties": {}}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		tasks := globalTaskStore.list()
		if len(tasks) == 0 {
			return "No tasks found.", nil
		}
		// Build resolved set for filtering BlockedBy display.
		resolvedIDs := map[string]bool{}
		for _, t := range tasks {
			if t.Status == taskStatusCompleted {
				resolvedIDs[t.ID] = true
			}
		}
		var lines []string
		for _, t := range tasks {
			owner := ""
			if t.Owner != "" {
				owner = " (" + t.Owner + ")"
			}
			var unresolved []string
			for _, b := range t.BlockedBy {
				if !resolvedIDs[b] {
					unresolved = append(unresolved, "#"+b)
				}
			}
			blocked := ""
			if len(unresolved) > 0 {
				blocked = " [blocked by " + joinStrings(unresolved) + "]"
			}
			lines = append(lines, fmt.Sprintf("#%s [%s] %s%s%s", t.ID, t.Status, t.Subject, owner, blocked))
		}
		return joinLines(lines), nil
	},
}

// TaskUpdateTool definition - matches CC-Source TaskUpdateTool.
var TaskUpdateTool = Tool{
	Name:        "TaskUpdate",
	Description: "Update a task. Supports changing subject, description, status, owner, and metadata.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"taskId":      {"type": "string", "description": "The ID of the task to update"},
			"subject":     {"type": "string", "description": "New subject for the task"},
			"description": {"type": "string", "description": "New description for the task"},
			"activeForm":  {"type": "string", "description": "Present continuous form shown in spinner"},
			"status": {
				"type": "string",
				"enum": ["pending", "in_progress", "completed", "deleted"],
				"description": "New status for the task ('deleted' removes the task)"
			},
			"addBlocks":   {"type": "array", "items": {"type": "string"}, "description": "Task IDs this task blocks"},
			"addBlockedBy":{"type": "array", "items": {"type": "string"}, "description": "Task IDs that block this task"},
			"owner":       {"type": "string", "description": "New owner for the task"},
			"metadata":    {"type": "object", "description": "Metadata keys to merge. Set a key to null to delete it."}
		},
		"required": ["taskId"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			TaskID      string         `json:"taskId"`
			Subject     string         `json:"subject"`
			Description string         `json:"description"`
			ActiveForm  string         `json:"activeForm"`
			Status      string         `json:"status"`
			AddBlocks   []string       `json:"addBlocks"`
			AddBlockedBy []string      `json:"addBlockedBy"`
			Owner       string         `json:"owner"`
			Metadata    map[string]any `json:"metadata"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}
		if input.TaskID == "" {
			// backward compat: also try task_id
			var alt struct {
				TaskID string `json:"task_id"`
				Status string `json:"status"`
			}
			_ = json.Unmarshal(args, &alt)
			input.TaskID = alt.TaskID
			if input.Status == "" {
				input.Status = alt.Status
			}
		}

		updated, err := globalTaskStore.update(
			input.TaskID,
			input.Subject,
			input.Description,
			input.ActiveForm,
			input.Status,
			input.Owner,
			input.AddBlocks,
			input.AddBlockedBy,
			input.Metadata,
		)
		if err != nil {
			return "", err
		}
		if input.Status == taskStatusDeleted {
			return fmt.Sprintf("Task #%s deleted.", input.TaskID), nil
		}
		b, _ := json.Marshal(map[string]interface{}{
			"success": true,
			"taskId":  updated.ID,
			"status":  updated.Status,
		})
		return string(b), nil
	},
}

func joinStrings(ss []string) string {
	result := ""
	for i, s := range ss {
		if i > 0 {
			result += ", "
		}
		result += s
	}
	return result
}

func joinLines(ss []string) string {
	result := ""
	for i, s := range ss {
		if i > 0 {
			result += "\n"
		}
		result += s
	}
	return result
}
