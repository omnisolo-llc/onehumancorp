package local

import (
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
	"time"

	"github.com/onehumancorp/mono/srcs/server/tools/registry"
)

var _ registry.AgentTool = (*BashTool)(nil)

type BashTool struct {
	workDir string
}

func NewBashTool(workDir string) *BashTool {
	return &BashTool{workDir: workDir}
}

func (t *BashTool) Name() string { return "bash" }

func (t *BashTool) Description() string {
	return "Execute a bash command in a shell. Use for running programs, scripts, build commands, or any shell operation. Commands run synchronously and output is returned."
}

func (t *BashTool) InputSchema() json.RawMessage {
	return json.RawMessage(`{
		"type": "object",
		"properties": {
			"command": {
				"type": "string",
				"description": "The bash command to execute."
			},
			"timeout": {
				"type": "integer",
				"description": "Optional timeout in seconds (default 120)."
			}
		},
		"required": ["command"]
	}`)
}

type BashInput struct {
	Command string `json:"command"`
	Timeout int    `json:"timeout,omitempty"`
}

type BashOutput struct {
	Output string `json:"output"`
	Error  string `json:"error,omitempty"`
}

func (t *BashTool) Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error) {
	var params BashInput
	if err := json.Unmarshal(input, &params); err != nil {
		return nil, fmt.Errorf("invalid input: %w", err)
	}

	if params.Command == "" {
		return nil, fmt.Errorf("bash: command is required")
	}

	timeoutSec := params.Timeout
	if timeoutSec == 0 {
		timeoutSec = 120
	}
	timeoutDur := time.Duration(timeoutSec) * time.Second

	execCtx, cancel := context.WithTimeout(ctx, timeoutDur)
	defer cancel()

	cmd := exec.CommandContext(execCtx, "bash", "-c", params.Command)
	cmd.Dir = t.workDir
	out, err := cmd.CombinedOutput()

	result := BashOutput{
		Output: string(out),
	}

	if err != nil {
		result.Error = err.Error()
	}

	return json.Marshal(result)
}
