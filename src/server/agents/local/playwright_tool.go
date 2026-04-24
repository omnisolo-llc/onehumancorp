package local

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
)

type playwrightTool struct {
	daemonURL string
}

func (t *playwrightTool) Definition() ToolDefinition {
	return ToolDefinition{
		Name:        "playwright",
		Description: "Interact with a web browser using Playwright. Useful for E2E testing and scraping.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"action": map[string]interface{}{
					"type":        "string",
					"description": "Action to perform: goto, eval, content.",
					"enum":        []string{"goto", "eval", "content"},
				},
				"url": map[string]interface{}{
					"type":        "string",
					"description": "URL to navigate to (required for 'goto' action).",
				},
				"script": map[string]interface{}{
					"type":        "string",
					"description": "JavaScript to evaluate (required for 'eval' action).",
				},
			},
			"required": []string{"action"},
		},
	}
}

func (t *playwrightTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
	action := strArg(input, "action")
	if action == "" {
		return "", errors.New("playwright: action is required")
	}

	var reqType, reqCommand string

	switch action {
	case "goto":
		url := strArg(input, "url")
		if url == "" {
			return "", errors.New("playwright: url is required for goto action")
		}
		reqType = "goto"
		reqCommand = url
	case "eval":
		script := strArg(input, "script")
		if script == "" {
			return "", errors.New("playwright: script is required for eval action")
		}
		reqType = "eval"
		reqCommand = script
	case "content":
		reqType = "content"
	default:
		return "", fmt.Errorf("playwright: unknown action %q", action)
	}

	reqBody := map[string]string{
		"type":    reqType,
		"command": reqCommand,
	}
	bodyBytes, err := json.Marshal(reqBody)
	if err != nil {
		return "", fmt.Errorf("playwright: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, "POST", t.daemonURL+"/command", bytes.NewBuffer(bodyBytes))
	if err != nil {
		return "", fmt.Errorf("playwright: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return "", fmt.Errorf("playwright: %w", err)
	}
	defer resp.Body.Close()

	var result struct {
		Stdout   string `json:"stdout"`
		Stderr   string `json:"stderr"`
		ExitCode int    `json:"exit_code"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return "", fmt.Errorf("playwright: decode response: %w", err)
	}

	if result.ExitCode != 0 {
		return "", fmt.Errorf("playwright error: %s", result.Stderr)
	}
	return result.Stdout, nil
}
