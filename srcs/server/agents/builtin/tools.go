package builtin

import (
	"context"
	"encoding/json"
)

// Tool represents an executable function the agent can call.
type Tool struct {
	Name        string
	Description string
	Parameters  json.RawMessage // JSON Schema of parameters
	RequiresAuth bool           // Indicates if tool needs user permission
	Execute     func(ctx context.Context, args json.RawMessage) (string, error)
}
