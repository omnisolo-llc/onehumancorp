package builtin

import (
	"context"
	"encoding/json"
	"strings"
)

// ToolSearchTool definition
var ToolSearchTool = Tool{
	Name:        "ToolSearch",
	Description: "Search for available tools and their descriptions.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"query": {
				"type": "string",
				"description": "Optional search query to filter tools."
			}
		}
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Query string `json:"query"`
		}
		if len(args) > 0 {
			if err := json.Unmarshal(args, &input); err != nil {
				return "", err
			}
		}

		// Hardcoded list of tools since we can't easily access the agent's actual tool list
		// from within the tool definition without a global registry or passing it in.
		tools := []struct {
			Name string
			Desc string
		}{
			{"Bash", "Execute a bash script."},
			{"Read", "Read a file from the local filesystem."},
			{"Write", "Write content to a file on the local filesystem."},
			{"Glob", "List files matching a glob pattern."},
			{"Grep", "Search for a pattern in files in the specified directory."},
			{"WebFetch", "Fetch the content of a URL."},
			{"WebSearch", "Search the web for a query."},
			{"SendMessage", "Send a message to the user."},
			{"TodoWrite", "Write to the active TODO list."},
			{"ToolSearch", "Search for available tools and their descriptions."},
		}

		var results []string
		for _, t := range tools {
			if input.Query == "" || strings.Contains(strings.ToLower(t.Name), strings.ToLower(input.Query)) || strings.Contains(strings.ToLower(t.Desc), strings.ToLower(input.Query)) {
				results = append(results, t.Name+": "+t.Desc)
			}
		}

		if len(results) == 0 {
			return "No tools found matching query.", nil
		}

		return strings.Join(results, "\n"), nil
	},
}
