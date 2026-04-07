package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

// WebFetchTool definition
var WebFetchTool = Tool{
	Name:        "WebFetch",
	Description: "Fetch the content of a URL.",
	SearchHint: "built-in tool",
	RequiresApproval: false,
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"url": {
				"type": "string",
				"description": "The URL to fetch."
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

		req, err := http.NewRequestWithContext(ctx, http.MethodGet, input.URL, nil)
		if err != nil {
			return "", err
		}

		resp, err := http.DefaultClient.Do(req)
		if err != nil {
			return "", err
		}
		defer resp.Body.Close()

		if resp.StatusCode != http.StatusOK {
			return "", fmt.Errorf("unexpected status code: %d", resp.StatusCode)
		}

		body, err := io.ReadAll(resp.Body)
		if err != nil {
			return "", err
		}

		return string(body), nil
	},
}