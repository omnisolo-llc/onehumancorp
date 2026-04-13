package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type Decomposer struct {
    llmClient MinimaxClient
    taskManager *TaskManager
    mutexProvider MutexProvider
}

func NewDecomposer(llm MinimaxClient, tm *TaskManager, mp MutexProvider) *Decomposer {
    return &Decomposer{
        llmClient: llm,
        taskManager: tm,
        mutexProvider: mp,
    }
}

type SubTaskSchema struct {
    Title       string   `json:"title"`
    Description string   `json:"description"`
    AgentRole   string   `json:"agent_role"`
    Priority    string   `json:"priority"`
    DependsOn   []string `json:"depends_on"`
}

func (d *Decomposer) DecomposeTask(ctx context.Context, organizationID, parentPlanID, prompt string) error {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        claims = &auth.Claims{OrganizationID: organizationID}
        ctx = auth.ContextWithClaims(ctx, claims)
    }

    lockKey := fmt.Sprintf("decompose:%s:%s", organizationID, parentPlanID)
    mutex := d.mutexProvider.NewMutex(lockKey)
    if err := mutex.Lock(ctx, 30*time.Second); err != nil {
        return fmt.Errorf("failed to acquire decompose lock: %w", err)
    }
    defer mutex.Unlock(ctx)

    systemPrompt := fmt.Sprintf("Break down the following request into a list of tasks. Respond strictly with a JSON array of objects with keys: title, description, agent_role, priority (P0, P1, P2), depends_on (array of titles). Request: %s", prompt)

    response, err := d.llmClient.Reason(ctx, systemPrompt)
    if err != nil {
        return fmt.Errorf("LLM reason error: %w", err)
    }

    // Clean up markdown block if present
    response = strings.TrimSpace(response)
    if strings.HasPrefix(response, "```json") {
        response = strings.TrimPrefix(response, "```json")
        response = strings.TrimSuffix(response, "```")
    } else if strings.HasPrefix(response, "```") {
        response = strings.TrimPrefix(response, "```")
        response = strings.TrimSuffix(response, "```")
    }

    var subTasks []SubTaskSchema
    if err := json.Unmarshal([]byte(response), &subTasks); err != nil {
        return fmt.Errorf("failed to parse LLM response: %w", err)
    }

    createdTasks := make(map[string]*SharedTask)

    for _, st := range subTasks {
        task, err := d.taskManager.CreateTaskWithPlan(ctx, organizationID, nil, st.Title, st.Description, st.Priority)
        if err != nil {
            slog.Error("Failed to create subtask", "title", st.Title, "error", err)
            continue
        }

        // We manually update parentPlanID since CreateTaskWithPlan lacks it in signature but struct has it
        if parentPlanID != "" {
            updateQuery := "UPDATE shared_tasks SET parent_plan_id = $1 WHERE id = $2"
            _, dbErr := d.taskManager.db.Exec(ctx, updateQuery, parentPlanID, task.ID)
            if dbErr != nil {
                slog.Error("Failed to update parent_plan_id", "task", task.ID, "error", dbErr)
            }
        }

        if d.taskManager.hub != nil {
            d.taskManager.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
                "action": "CREATE_SUBTASK",
                "title": st.Title,
                "agent_role": st.AgentRole,
                "parent_plan_id": parentPlanID,
            })
        }

        err = d.taskManager.DelegateSubTask(ctx, task.ID, st.AgentRole, map[string]interface{}{
            "title": st.Title,
            "description": st.Description,
        })
        if err != nil {
            slog.Error("Failed to delegate subtask", "task_id", task.ID, "error", err)
        }

        createdTasks[st.Title] = task
    }

    for _, st := range subTasks {
        task, ok := createdTasks[st.Title]
        if !ok { continue }

        for _, depTitle := range st.DependsOn {
            depTask, ok := createdTasks[depTitle]
            if ok {
                query := "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)"
                _, err := d.taskManager.db.Exec(ctx, query, task.ID, depTask.ID)
                if err != nil {
                    slog.Error("Failed to add dependency", "task", task.ID, "dep", depTask.ID, "error", err)
                }
            }
        }
    }

    return nil
}
