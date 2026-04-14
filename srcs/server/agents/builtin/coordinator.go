package builtin

import (
	"context"
	"fmt"
	"strings"
	"sync"
	"time"
)

// CoordinatorMode defines how a coordinator agent manages sub-agents.
// Mirrors CC-Source's coordinator mode where the coordinator's only job is to
// spawn agents, stop agents, and route messages between them.
type CoordinatorMode struct {
	// MaxConcurrentAgents is the ceiling on simultaneously running sub-agents.
	// 0 means unlimited.
	MaxConcurrentAgents int
}

// CoordinatorAgent is an agent that only uses CoordinatorTools to manage a swarm.
// It has a restricted tool set: AgentTool, TaskStopTool, TaskStatusTool, SendMessage.
type CoordinatorAgent struct {
	BuiltinAgent
	mode   CoordinatorMode
	mu     sync.Mutex
	agents map[string]*SubagentState // keyed by taskID
}

// NewCoordinatorAgent creates a coordinator agent.
func NewCoordinatorAgent(client LLMClient, model, systemPrompt string, mode CoordinatorMode) *CoordinatorAgent {
	return &CoordinatorAgent{
		BuiltinAgent: BuiltinAgent{
			Client:      client,
			Model:       model,
			System:      systemPrompt,
			Tools:       CoordinatorTools(),
			MaxTokens:   8192,
			Temperature: 0.1,
		},
		mode:   mode,
		agents: make(map[string]*SubagentState),
	}
}

// RunCoordinator runs the coordinator agent loop.
// It blocks until the coordinator produces a final answer.
func (c *CoordinatorAgent) RunCoordinator(ctx context.Context, directives []string) (string, error) {
	// Build the initial prompt from directives.
	prompt := buildCoordinatorPrompt(directives)
	messages, err := c.Run(ctx, []Message{{Role: RoleUser, Content: prompt}})
	if err != nil {
		return "", fmt.Errorf("coordinator: %w", err)
	}
	// Return the last assistant message content.
	for i := len(messages) - 1; i >= 0; i-- {
		if messages[i].Role == RoleAssistant && messages[i].Content != "" {
			return messages[i].Content, nil
		}
	}
	return "", nil
}

// buildCoordinatorPrompt creates the system prompt for coordinator mode.
func buildCoordinatorPrompt(directives []string) string {
	var sb strings.Builder
	sb.WriteString("You are a coordinator agent. Your role is to:\n")
	sb.WriteString("1. Analyze the work to be done\n")
	sb.WriteString("2. Break it down into parallel sub-tasks\n")
	sb.WriteString("3. Use the Agent tool to spawn sub-agents for each sub-task\n")
	sb.WriteString("4. Wait for results via task-notification messages\n")
	sb.WriteString("5. Aggregate the results and provide a final summary\n\n")
	sb.WriteString("You MUST use the Agent tool to delegate work — do NOT do the work yourself.\n\n")
	sb.WriteString("Directives:\n")
	for i, d := range directives {
		sb.WriteString(fmt.Sprintf("%d. %s\n", i+1, d))
	}
	return sb.String()
}

// ForkAgent represents a sub-agent forked from a parent conversation context.
// Mirrors CC-Source's fork subagent pattern where the child inherits the parent's context.
type ForkAgent struct {
	// Directive is the specific task for this forked agent.
	Directive string
	// ParentContext is the conversation history up to the fork point.
	ParentContext []Message
	// WorkDir is the working directory (may differ if using isolated worktree).
	WorkDir string
}

// ForkChildMessage builds the first message for a forked sub-agent.
// Mirrors CC-Source's buildChildMessage + buildForkedMessages.
// The child receives:
//  1. Full parent conversation history up to fork point
//  2. A boilerplate message with its directive
func ForkChildMessage(directive, parentCwd string) Message {
	return Message{
		Role: RoleUser,
		Content: fmt.Sprintf(`<fork-boilerplate>
STOP. READ THIS FIRST.

You are a forked worker process. You are NOT the main agent.

RULES (non-negotiable):
1. Do NOT spawn sub-agents; execute directly.
2. Do NOT converse, ask questions, or suggest next steps.
3. Do NOT editorialize or add meta-commentary.
4. USE your tools directly: Bash, Read, Write, etc.
5. If you modify files, commit your changes before reporting.
6. Do NOT emit text between tool calls. Use tools silently, then report once at the end.
7. Stay strictly within your directive's scope.
8. Keep your report under 500 words unless specified otherwise.
9. Your response MUST begin with "Scope:". No preamble, no thinking-out-loud.
10. REPORT structured facts, then stop

Output format (plain text labels, not markdown headers):
  Scope: <echo back your assigned scope in one sentence>
  Result: <the answer or key findings, limited to the scope above>
  Key files: <relevant file paths — include for research tasks>
  Files changed: <list with commit hash — include only if you modified files>
  Issues: <list — include only if there are issues to flag>
</fork-boilerplate>

Your directive: %s`, directive),
	}
}

// ProgressSummarizer generates periodic summaries of agent progress.
// Mirrors CC-Source's AgentSummary service which calls the LLM periodically
// to summarize a running agent's activity for display in the panel.
type ProgressSummarizer struct {
	client      LLMClient
	model       string
	interval    time.Duration
	mu          sync.Mutex
	lastSummary string
}

// NewProgressSummarizer creates a new summarizer that will call the LLM
// at the given interval to summarize agent progress.
func NewProgressSummarizer(client LLMClient, model string, interval time.Duration) *ProgressSummarizer {
	if interval <= 0 {
		interval = 30 * time.Second
	}
	return &ProgressSummarizer{
		client:   client,
		model:    model,
		interval: interval,
	}
}

// Start begins periodic summarization of agent activity.
// It reads recent activities from the progressFn callback and calls the LLM.
// The onSummary callback is called with each new summary.
// Blocks until ctx is cancelled.
func (s *ProgressSummarizer) Start(
	ctx context.Context,
	descriptionFn func() string,
	activitiesFn func() []string,
	onSummary func(string),
) {
	ticker := time.NewTicker(s.interval)
	defer ticker.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			summary := s.summarize(ctx, descriptionFn(), activitiesFn())
			if summary != "" {
				s.mu.Lock()
				s.lastSummary = summary
				s.mu.Unlock()
				onSummary(summary)
			}
		}
	}
}

// LastSummary returns the most recent generated summary.
func (s *ProgressSummarizer) LastSummary() string {
	s.mu.Lock()
	defer s.mu.Unlock()
	return s.lastSummary
}

func (s *ProgressSummarizer) summarize(ctx context.Context, description string, activities []string) string {
	if len(activities) == 0 {
		return ""
	}
	prompt := fmt.Sprintf(
		"Summarize in 1-2 sentences what this agent is currently doing:\n\nTask: %s\n\nRecent activities:\n%s",
		description,
		strings.Join(activities, "\n"),
	)
	resp, err := s.client.Chat(ctx, ChatRequest{
		Model:     s.model,
		System:    "You are a concise summarizer. Reply with 1-2 sentences only.",
		Messages:  []Message{{Role: RoleUser, Content: prompt}},
		MaxTokens: 150,
	})
	if err != nil {
		return ""
	}
	return strings.TrimSpace(resp.Message.Content)
}
