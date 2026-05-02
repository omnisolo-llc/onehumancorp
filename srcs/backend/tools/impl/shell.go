package impl

import (
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
)

// ShellTool executes shell commands.
type ShellTool struct{}

// NewShellTool creates a new ShellTool.
func NewShellTool() *ShellTool {
	return &ShellTool{}
}

// Name returns the name of the tool.
func (t *ShellTool) Name() string {
	return "shell"
}

// Description returns the description of the tool.
func (t *ShellTool) Description() string {
	return "Executes a shell command and returns the output."
}

// InputSchema returns the JSON schema for the tool's input.
func (t *ShellTool) InputSchema() json.RawMessage {
	return []byte(`{
		"type": "object",
		"properties": {
			"command": {
				"type": "string",
				"description": "The shell command to execute."
			}
		},
		"required": ["command"]
	}`)
}

type shellInput struct {
	Command string `json:"command"`
}

type shellOutput struct {
	Stdout string `json:"stdout"`
	Stderr string `json:"stderr,omitempty"`
	Error  string `json:"error,omitempty"`
}

// Execute runs the shell command.
func (t *ShellTool) Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error) {
	var in shellInput
	if err := json.Unmarshal(input, &in); err != nil {
		return nil, fmt.Errorf("invalid input for shell tool: %w", err)
	}

	cmd := exec.CommandContext(ctx, "sh", "-c", in.Command)
	out, err := cmd.CombinedOutput()

	output := shellOutput{
		Stdout: string(out),
	}

	if err != nil {
		output.Error = err.Error()
	}

	return json.Marshal(output)
}
