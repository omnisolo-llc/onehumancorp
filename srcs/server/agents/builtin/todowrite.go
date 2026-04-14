package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"sync"
)

// todoItem is a single entry in the todo list.
type todoItem struct {
	Content string `json:"content"`
	Status  string `json:"status"` // "pending", "in_progress", "completed"
	// Priority is optional context; not required.
	Priority string `json:"priority,omitempty"`
}

// per-process in-memory todo list, mirrors CC-Source TodoWriteTool.
var todoMu sync.RWMutex
var todoList []todoItem

// TodoWriteTool replaces the entire todo list with the provided items.
// Mirrors CC-Source's TodoWriteTool which accepts a full list and overwrites.
var TodoWriteTool = Tool{
	Name: "TodoWrite",
	Description: "Create or update the session todo list. Provide the full " +
		"updated list — this replaces the previous list entirely. " +
		"Each item must have 'content' and 'status' (pending/in_progress/completed). " +
		"Use this tool to track tasks and progress throughout the session.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"todos": {
				"type": "array",
				"description": "The complete updated todo list",
				"items": {
					"type": "object",
					"properties": {
						"content": {"type": "string", "description": "Task description"},
						"status": {
							"type": "string",
							"enum": ["pending", "in_progress", "completed"],
							"description": "Task status"
						},
						"priority": {"type": "string", "description": "Optional priority hint"}
					},
					"required": ["content", "status"]
				}
			}
		},
		"required": ["todos"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Todos []todoItem `json:"todos"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", fmt.Errorf("TodoWrite: invalid args: %w", err)
		}

		// Validate statuses.
		for i, t := range input.Todos {
			switch t.Status {
			case "pending", "in_progress", "completed":
				// valid
			case "":
				input.Todos[i].Status = "pending"
			default:
				return "", fmt.Errorf("TodoWrite: invalid status %q for item %d", t.Status, i)
			}
		}

		todoMu.Lock()
		todoList = input.Todos
		todoMu.Unlock()

		// Also persist to disk for durability across process restarts.
		if err := persistTodoList(input.Todos); err != nil {
			// Non-fatal: in-memory list is updated.
			_ = err
		}

		allDone := true
		for _, t := range input.Todos {
			if t.Status != "completed" {
				allDone = false
				break
			}
		}

		var sb strings.Builder
		sb.WriteString("Todos updated successfully.\n")
		for _, t := range input.Todos {
			icon := "○"
			switch t.Status {
			case "in_progress":
				icon = "◉"
			case "completed":
				icon = "✓"
			}
			sb.WriteString(fmt.Sprintf("%s %s\n", icon, t.Content))
		}
		if allDone && len(input.Todos) > 0 {
			sb.WriteString("\nAll tasks completed.")
		}
		return sb.String(), nil
	},
}

// TodoReadTool returns the current todo list.
var TodoReadTool = Tool{
	Name:        "TodoRead",
	Description: "Read the current session todo list.",
	Parameters:  json.RawMessage(`{"type": "object", "properties": {}}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		todoMu.RLock()
		current := todoList
		todoMu.RUnlock()

		if len(current) == 0 {
			return "No todos.", nil
		}
		var sb strings.Builder
		for i, t := range current {
			icon := "○"
			switch t.Status {
			case "in_progress":
				icon = "◉"
			case "completed":
				icon = "✓"
			}
			sb.WriteString(fmt.Sprintf("%d. %s %s\n", i+1, icon, t.Content))
		}
		return sb.String(), nil
	},
}

// todoFilePath returns the path for the todo file.
// It uses OHC_TODO_FILE env var when set; otherwise a per-process temp file
// so multiple concurrent agent runs don't interfere with each other.
func todoFilePath() string {
	if p := os.Getenv("OHC_TODO_FILE"); p != "" {
		return p
	}
	return filepath.Join(os.TempDir(), "ohc-todo-"+strconv.Itoa(os.Getpid())+".json")
}

// persistTodoList writes the todo list to disk as JSON.
func persistTodoList(todos []todoItem) error {
	todoPath := todoFilePath()
	if err := os.MkdirAll(filepath.Dir(todoPath), 0o755); err != nil {
		return err
	}
	b, err := json.MarshalIndent(todos, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(todoPath, b, 0o644)
}