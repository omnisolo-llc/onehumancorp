package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
)

// Task represents a task in the system.
type Task struct {
	ID          string    `json:"id"`
	Subject     string    `json:"subject"`
	Description string    `json:"description"`
	Status      string    `json:"status"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

// Memory task store for demonstration. In a real system, this would be a database.
var taskStore = make(map[string]*Task)
var taskCounter = 0

// TaskCreateTool definition
var TaskCreateTool = Tool{
	Name:        "TaskCreate",
	Description: "Create a new task.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"subject": {
				"type": "string",
				"description": "A brief title for the task"
			},
			"description": {
				"type": "string",
				"description": "What needs to be done"
			}
		},
		"required": ["subject", "description"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Subject     string `json:"subject"`
			Description string `json:"description"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		taskCounter++
		id := fmt.Sprintf("task-%d", taskCounter)

		task := &Task{
			ID:          id,
			Subject:     input.Subject,
			Description: input.Description,
			Status:      "todo",
			CreatedAt:   time.Now().UTC(),
			UpdatedAt:   time.Now().UTC(),
		}

		taskStore[id] = task

		output, err := json.Marshal(task)
		if err != nil {
			return "", err
		}
		return string(output), nil
	},
}

// TaskGetTool definition
var TaskGetTool = Tool{
	Name:        "TaskGet",
	Description: "Get details of a specific task.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"task_id": {
				"type": "string",
				"description": "The ID of the task"
			}
		},
		"required": ["task_id"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			TaskID string `json:"task_id"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		task, ok := taskStore[input.TaskID]
		if !ok {
			return "", fmt.Errorf("task with ID %s not found", input.TaskID)
		}

		output, err := json.Marshal(task)
		if err != nil {
			return "", err
		}
		return string(output), nil
	},
}

// TaskListTool definition
var TaskListTool = Tool{
	Name:        "TaskList",
	Description: "List all tasks.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {}
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var tasks []*Task
		for _, task := range taskStore {
			tasks = append(tasks, task)
		}

		if len(tasks) == 0 {
			return "[]", nil
		}

		output, err := json.Marshal(tasks)
		if err != nil {
			return "", err
		}
		return string(output), nil
	},
}

// TaskUpdateTool definition
var TaskUpdateTool = Tool{
	Name:        "TaskUpdate",
	Description: "Update a task.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"task_id": {
				"type": "string",
				"description": "The ID of the task to update"
			},
			"status": {
				"type": "string",
				"description": "The new status of the task"
			}
		},
		"required": ["task_id", "status"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			TaskID string `json:"task_id"`
			Status string `json:"status"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		task, ok := taskStore[input.TaskID]
		if !ok {
			return "", fmt.Errorf("task with ID %s not found", input.TaskID)
		}

		task.Status = input.Status
		task.UpdatedAt = time.Now().UTC()

		output, err := json.Marshal(task)
		if err != nil {
			return "", err
		}
		return string(output), nil
	},
}
