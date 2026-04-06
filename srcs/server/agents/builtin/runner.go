package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"sync"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/agents/local"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// Runner manages the inbox and task execution for the builtin agent.
type Runner struct {
	hub       local.Hub
	agentID   string
	agentName string
	role      string
	cfg       AgentConfig

	mu     sync.Mutex
	cancel context.CancelFunc
}

func NewRunner(hub local.Hub, agentID, agentName, role string, cfg AgentConfig) *Runner {
	if agentID == "" {
		agentID = uuid.New().String()
	}
	if agentName == "" {
		agentName = "builtin-agent"
	}
	if role == "" {
		role = "SOFTWARE_ENGINEER"
	}
	return &Runner{
		hub:       hub,
		agentID:   agentID,
		agentName: agentName,
		role:      role,
		cfg:       cfg,
	}
}

func (r *Runner) Start(ctx context.Context) {
	ctx, cancel := context.WithCancel(ctx)
	r.cancel = cancel
	defer cancel()

	r.hub.RegisterAgent(local.HubAgent{
		ID:           r.agentID,
		Name:         r.agentName,
		Role:         r.role,
		Status:       local.HubStatusIdle,
		ProviderType: "builtin",
	})

	sig, unsub := r.hub.Subscribe(r.agentID)
	defer unsub()

	r.drainInbox(ctx)

	for {
		select {
		case <-ctx.Done():
			return
		case <-sig:
			r.drainInbox(ctx)
		}
	}
}

func (r *Runner) Stop() {
	if r.cancel != nil {
		r.cancel()
	}
}

func (r *Runner) drainInbox(ctx context.Context) {
	messages := r.hub.Inbox(r.agentID)
	for _, msg := range messages {
		if msg.Type == "TaskAssignment" {
			r.handleTaskAssignment(ctx, msg)
		}
	}
}

func (r *Runner) handleTaskAssignment(ctx context.Context, msg local.HubMessage) {
	var payload local.TaskAssignmentPayload
	if err := json.Unmarshal([]byte(msg.Content), &payload); err != nil {
		slog.Error("builtin runner: failed to parse TaskAssignment", "err", err)
		return
	}

	description := payload.IssueName
	prompt := payload.Prompt

	r.hub.RegisterAgent(local.HubAgent{
		ID:           r.agentID,
		Name:         r.agentName,
		Role:         r.role,
		Status:       local.HubStatusActive,
		ProviderType: "builtin",
	})

	go func() {
		defer func() {
			r.hub.RegisterAgent(local.HubAgent{
				ID:           r.agentID,
				Name:         r.agentName,
				Role:         r.role,
				Status:       local.HubStatusIdle,
				ProviderType: "builtin",
			})
		}()

		agent := NewAgent(r.cfg)
		result, err := agent.Run(ctx, prompt)

		status := "COMPLETED"
		var summaryText string
		if err != nil {
			status = "FAILED"
			summaryText = fmt.Sprintf("Task failed: %v", err)
		} else {
			summaryText = fmt.Sprintf("Task completed successfully. Result: %s", result)
		}

		replyPayload, _ := json.Marshal(map[string]string{
			"taskId":      payload.IssueID,
			"status":      status,
			"summary":     summaryText,
			"description": description,
		})

		_ = r.hub.Publish(local.HubMessage{
			ID:        uuid.New().String(),
			FromAgent: r.agentID,
			ToAgent:   msg.FromAgent,
			Type:      "TaskCompletion",
			Content:   string(replyPayload),
		})
	}()
}

func StartDefaultRunner(hub *orchestration.Hub, llm local.LLMClient) (*Runner, error) {
	if hub == nil {
		return nil, fmt.Errorf("StartDefaultRunner: hub must not be nil")
	}

	if llm == nil {
		llm = local.DefaultLLMClient()
	}

	adapter := NewOrchestrationHubAdapter(hub)
	cfg := AgentConfig{LLM: llm}
	runner := NewRunner(adapter, "", "", "", cfg)
	return runner, nil
}
