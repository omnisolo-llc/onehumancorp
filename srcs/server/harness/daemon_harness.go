package harness

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
)

// DaemonHarness implements AgentHarness by forwarding commands to the Harness Daemon.
type DaemonHarness struct {
	daemonURL string
}

// NewDaemonHarness creates a new DaemonHarness.
func NewDaemonHarness(url string) *DaemonHarness {
	return &DaemonHarness{
		daemonURL: url,
	}
}

// Execute parses the command and sends it to the daemon.
func (h *DaemonHarness) Execute(ctx context.Context, command string) (Result, error) {
	// Simple parsing for daemon protocol
	// In reality we would use structured tools, but this fits the AgentHarness interface
	var reqBody map[string]string

	cmd := strings.TrimSpace(command)
	if strings.HasPrefix(cmd, "playwright goto ") {
		reqBody = map[string]string{
			"type":    "goto",
			"command": strings.TrimPrefix(cmd, "playwright goto "),
		}
	} else if strings.HasPrefix(cmd, "playwright eval ") {
		reqBody = map[string]string{
			"type":    "eval",
			"command": strings.TrimPrefix(cmd, "playwright eval "),
		}
	} else if cmd == "playwright content" {
		reqBody = map[string]string{
			"type": "content",
		}
	} else {
		return Result{}, fmt.Errorf("unsupported daemon command: %s", cmd)
	}

	bodyBytes, err := json.Marshal(reqBody)
	if err != nil {
		return Result{}, fmt.Errorf("failed to encode request: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, "POST", h.daemonURL+"/command", bytes.NewBuffer(bodyBytes))
	if err != nil {
		return Result{}, fmt.Errorf("failed to build request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return Result{}, fmt.Errorf("daemon request failed: %w", err)
	}
	defer resp.Body.Close()

	var result struct {
		Stdout   string `json:"stdout"`
		Stderr   string `json:"stderr"`
		ExitCode int    `json:"exit_code"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return Result{}, fmt.Errorf("failed to decode response: %w", err)
	}

	return Result{
		Stdout:   result.Stdout,
		Stderr:   result.Stderr,
		ExitCode: result.ExitCode,
	}, nil
}
