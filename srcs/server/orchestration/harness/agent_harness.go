package harness

import (
	"bytes"
	"os/exec"
)

type AttemptResult struct {
	Stdout    string
	Stderr    string
	ExitCode  int
	Compacted bool
}

type AgentHarness interface {
	RunAttempt(cmd string) (*AttemptResult, error)
	Compact() error
	Reset() error
}

type AssistantAgentHarness struct {
	manager *AssistantSandboxManager
}

func NewAssistantAgentHarness(policyJSON string) (AgentHarness, error) {
	manager := NewAssistantSandboxManager()
	if policyJSON != "" {
		if err := manager.UpdateConfig(policyJSON); err != nil {
			return nil, err
		}
	}
	return &AssistantAgentHarness{manager: manager}, nil
}

func (h *AssistantAgentHarness) RunAttempt(cmd string) (*AttemptResult, error) {
	wrappedCmd, err := h.manager.WrapCommand(cmd)
	if err != nil {
		return nil, err
	}

	c := exec.Command("sh", "-c", wrappedCmd)
	var out bytes.Buffer
	var stderr bytes.Buffer
	c.Stdout = &out
	c.Stderr = &stderr

	err = c.Run()
	exitCode := 0
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCode = exitErr.ExitCode()
		} else {
			exitCode = -1
		}
	}

	return &AttemptResult{
		Stdout:   out.String(),
		Stderr:   stderr.String(),
		ExitCode: exitCode,
	}, nil
}

func (h *AssistantAgentHarness) Compact() error {
	return nil
}

func (h *AssistantAgentHarness) Reset() error {
	return nil
}
