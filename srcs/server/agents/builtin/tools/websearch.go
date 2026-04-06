package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
)

// WebSearchTool definition
var WebSearchTool = Tool{
	Name:        "WebSearch",
	Description: "Search the web for a query.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"query": {
				"type": "string",
				"description": "The search query."
			}
		},
		"required": ["query"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Query string `json:"query"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		// A real implementation would hit a search API.
		// We'll return a placeholder for now.
		return fmt.Sprintf("Simulated search results for: %s\nhttps://duckduckgo.com/html/?q=%s", input.Query, url.QueryEscape(input.Query)), nil
	},
}