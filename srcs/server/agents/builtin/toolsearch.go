package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
)

// toolManifest is a static manifest used by ToolSearch.
// It mirrors AllTools + coordinator tools without creating an init cycle.
var toolManifest = []struct{ Name, Desc string }{
	{"Bash", "Execute a bash command. Use for build/test/git/shell operations."},
	{"Read", "Read a file from the local filesystem."},
	{"Write", "Create or overwrite a file on the local filesystem."},
	{"Edit", "Edit a file: replace an exact string with a new string."},
	{"Glob", "List files matching a glob pattern."},
	{"Grep", "Search for a regex pattern in files."},
	{"WebFetch", "Fetch a URL and return its content as text."},
	{"WebSearch", "Search the web using DuckDuckGo."},
	{"SendMessage", "Send a message to the user or another agent."},
	{"TodoWrite", "Create or update the session todo list."},
	{"TodoRead", "Read the current session todo list."},
	{"ToolSearch", "Search available tools by name or description."},
	{"TaskCreate", "Create a new task in the task list."},
	{"TaskGet", "Get details of a task by ID."},
	{"TaskList", "List all tasks."},
	{"TaskUpdate", "Update a task (subject, description, status, owner, etc.)."},
	{"Sleep", "Sleep for N seconds; use while waiting for CI/builds."},
	{"Agent", "Spawn a background sub-agent to perform a task concurrently."},
	{"TaskStop", "Stop a running background agent task."},
	{"TaskStatus", "Get the status and progress of a background agent task."},
}

// ToolSearchTool definition - returns available tools matching an optional query.
// Mirrors CC-Source's ToolSearchTool which searches available tool definitions.
var ToolSearchTool = Tool{
	Name: "ToolSearch",
	Description: "Search for available tools and their descriptions. " +
		"Use this when you are unsure which tool to use for a given task.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"query": {
				"type": "string",
				"description": "Optional search query to filter tools by name or description."
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

		q := strings.ToLower(input.Query)
		var results []string
		for _, t := range toolManifest {
			if q == "" ||
				strings.Contains(strings.ToLower(t.Name), q) ||
				strings.Contains(strings.ToLower(t.Desc), q) {
				results = append(results, fmt.Sprintf("%-16s %s", t.Name+":", t.Desc))
			}
		}

		if len(results) == 0 {
			return "No tools found matching query.", nil
		}

		return strings.Join(results, "\n"), nil
	},
}