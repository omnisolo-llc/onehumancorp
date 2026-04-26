package builtin

import (
	"context"
	"encoding/json"
)

type Tool struct {
	Name        string
	Description string
	Parameters  json.RawMessage
	Execute     func(ctx context.Context, args json.RawMessage) (string, error)
}
