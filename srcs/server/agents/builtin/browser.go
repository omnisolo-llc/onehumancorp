package builtin

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
)

// BrowserTool uses a persistent browser daemon to execute browser commands.
var BrowserTool = Tool{
	Name:        "Browser",
	Description: "Execute browser commands through a persistent daemon to navigate the web, preserving state.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"url": {
				"type": "string",
				"description": "The URL to navigate to."
			}
		},
		"required": ["url"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			URL string `json:"url"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}
		if input.URL == "" {
			return "", fmt.Errorf("Browser: url is required")
		}

		reqBody, err := json.Marshal(input)
		if err != nil {
			return "", fmt.Errorf("Browser: serialize request: %w", err)
		}

		// Connect to local daemon
		req, err := http.NewRequestWithContext(ctx, http.MethodPost, "http://localhost:9222/command", bytes.NewBuffer(reqBody))
		if err != nil {
			return "", fmt.Errorf("Browser: new request: %w", err)
		}
		req.Header.Set("Content-Type", "application/json")

		resp, err := http.DefaultClient.Do(req)
		if err != nil {
			return "", fmt.Errorf("Browser: do request: %w", err)
		}
		defer resp.Body.Close()

		if resp.StatusCode >= 400 {
			return "", fmt.Errorf("Browser: HTTP %d from daemon", resp.StatusCode)
		}

		var result struct {
			Content string `json:"content"`
			Error   string `json:"error,omitempty"`
		}
		if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
			return "", fmt.Errorf("Browser: decode response: %w", err)
		}

		if result.Error != "" {
			return "", fmt.Errorf("Browser daemon error: %s", result.Error)
		}

		return result.Content, nil
	},
}
