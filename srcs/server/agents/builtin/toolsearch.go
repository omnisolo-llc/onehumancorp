package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
)

// toolManifest is a static manifest used by ToolSearch.
// It mirrors AllTools + coordinator tools without creating an init cycle.
var toolManifest = []struct{ Name, Desc, Params string }{
	{"Bash", "Execute a bash command. Use for build/test/git/shell operations.", ""},
	{"Read", "Read a file from the local filesystem.", ""},
	{"Write", "Create or overwrite a file on the local filesystem.", ""},
	{"Edit", "Edit a file: replace an exact string with a new string.", ""},
	{"Glob", "List files matching a glob pattern.", ""},
	{"Grep", "Search for a regex pattern in files.", ""},
	{"WebFetch", "Fetch a URL and return its content as text.", ""},
	{"WebSearch", "Search the web using DuckDuckGo.", ""},
	{"SendMessage", "Send a message to the user or another agent.", ""},
	{"TodoWrite", "Create or update the session todo list.", ""},
	{"TodoRead", "Read the current session todo list.", ""},
	{"ToolSearch", "Search available tools by name or description.", ""},
	{"TaskCreate", "Create a new task in the task list.", ""},
	{"TaskGet", "Get details of a task by ID.", ""},
	{"TaskList", "List all tasks.", ""},
	{"TaskUpdate", "Update a task (subject, description, status, owner, etc.).", ""},
	{"Sleep", "Sleep for N seconds; use while waiting for CI/builds.", ""},
	{"Agent", "Spawn a background sub-agent to perform a task concurrently.", ""},
	{"TaskStop", "Stop a running background agent task.", ""},
	{"TaskStatus", "Get the status and progress of a background agent task.", ""},
}

func getAllToolsManifest() []struct{ Name, Description, Parameters string } {
	var res []struct{ Name, Description, Parameters string }
	for _, m := range toolManifest {
		res = append(res, struct{ Name, Description, Parameters string }{m.Name, m.Desc, m.Params})
	}
	return res
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

		isSelect := strings.HasPrefix(q, "select:")
		var requiredNames []string
		if isSelect {
			parts := strings.Split(strings.TrimPrefix(q, "select:"), ",")
			for _, p := range parts {
				p = strings.TrimSpace(p)
				if p != "" {
					requiredNames = append(requiredNames, strings.ToLower(p))
				}
			}
		}

		var results []string
		manifest := getAllToolsManifest()
		seen := make(map[string]bool)
		for _, t := range manifest {
			if seen[t.Name] {
				continue
			}
			seen[t.Name] = true

			match := false
			if isSelect {
				for _, req := range requiredNames {
					if strings.ToLower(t.Name) == req {
						match = true
						break
					}
				}
			} else {
				if q == "" ||
					strings.Contains(strings.ToLower(t.Name), q) ||
					strings.Contains(strings.ToLower(t.Description), q) {
					match = true
				}
			}

			if match {
				desc := t.Description
				schema := string(t.Parameters)
				if schema == "" {
					schema = "{}"
				}
				results = append(results, fmt.Sprintf("<function>{\"name\": %q, \"description\": %q, \"parameters\": %s}</function>", t.Name, desc, schema))
			}
		}

		if len(results) == 0 {
			return "No tools found matching query.", nil
		}

		return fmt.Sprintf("<functions>\n%s\n</functions>", strings.Join(results, "\n")), nil
	},
}
