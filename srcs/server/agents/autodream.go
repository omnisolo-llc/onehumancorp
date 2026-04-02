package agents

import (
	"context"
	"encoding/json"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// AutoDreamEngine periodically sweeps completed swarm_tasks and consolidates them into swarm_long_term_memory.
type AutoDreamEngine struct {
	hub           *orchestration.Hub
	pollInterval  time.Duration
	minimaxAPIKey string
}

func NewAutoDreamEngine(hub *orchestration.Hub, minimaxAPIKey string) *AutoDreamEngine {
	return &AutoDreamEngine{
		hub:           hub,
		pollInterval:  24 * time.Hour,
		minimaxAPIKey: minimaxAPIKey,
	}
}

func (e *AutoDreamEngine) Start(ctx context.Context) {
	ticker := time.NewTicker(e.pollInterval)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				e.consolidateMemories(ctx)
			}
		}
	}()
}

func (e *AutoDreamEngine) consolidateMemories(ctx context.Context) {
	sip := e.hub.SIPDB()
	if sip == nil {
		return
	}

	since := time.Now().UTC().Add(-24 * time.Hour)
	tasks, err := sip.GetCompletedTasksToConsolidate(ctx, since)
	if err != nil {
		slog.Error("autodream: failed to fetch completed tasks", "err", err)
		return
	}

	if len(tasks) == 0 {
		return
	}

	// Synthesize using LLM
	client := orchestration.NewMinimaxClient(e.minimaxAPIKey)

	tasksBytes, _ := json.Marshal(tasks)
	prompt := "You are AutoDream, an agentic memory consolidation process. Summarize the following COMPLETED task payloads into a dense architectural learning and long-term memory topic and summary. Output JSON like: {\"topic\": \"...\", \"summary\": \"...\"}\nTasks: " + string(tasksBytes)

	slog.Info("autodream: consolidating memories", "tasks_count", len(tasks))

	resp, err := client.Reason(ctx, prompt)
	if err != nil {
		slog.Error("autodream: summarization failed", "err", err)
		return
	}

	var result struct {
		Topic   string `json:"topic"`
		Summary string `json:"summary"`
	}

	if err := json.Unmarshal([]byte(resp), &result); err != nil {
		slog.Error("autodream: failed to parse llm response", "err", err)
		return
	}

	// Store consolidated memory
	if err := sip.StoreAutoDreamMemory(ctx, result.Topic, result.Summary, nil); err != nil {
		slog.Error("autodream: failed to store memory", "err", err)
		return
	}

	slog.Info("autodream: successfully consolidated memory", "topic", result.Topic)
}
