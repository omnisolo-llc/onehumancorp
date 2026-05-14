package harness

import (
	"bytes"
	"os/exec"

	apiharness "onehumancorp/src/server/api/harness"
)

type AssistantAgentHarness struct {
	manager *AssistantSandboxManager
}

func NewAssistantAgentHarness(policyJSON string) (apiharness.AgentHarness, error) {
	manager := NewAssistantSandboxManager()
	if policyJSON != "" {
		if err := manager.UpdateConfig(policyJSON); err != nil {
			return nil, err
		}
	}
	return &AssistantAgentHarness{manager: manager}, nil
}

func (h *AssistantAgentHarness) RunAttempt(cmd string) (*apiharness.AttemptResult, error) {
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

	return &apiharness.AttemptResult{
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
