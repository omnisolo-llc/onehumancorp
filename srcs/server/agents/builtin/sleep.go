package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
)

// SleepTool pauses execution for a specified duration.
// Mirrors CC-Source's SleepTool which allows the agent to yield time
// while waiting for long-running operations (e.g. waiting for CI to finish).
var SleepTool = Tool{
	Name: "Sleep",
	Description: "Sleep for a specified duration. Use this to wait for " +
		"long-running operations (e.g. waiting for CI to complete) rather " +
		"than polling in a tight loop.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"seconds": {
				"type": "number",
				"description": "Number of seconds to sleep (max 300)."
			}
		},
		"required": ["seconds"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Seconds float64 `json:"seconds"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", fmt.Errorf("Sleep: invalid args: %w", err)
		}
		if input.Seconds <= 0 {
			return "Slept 0s.", nil
		}
		if input.Seconds > 300 {
			input.Seconds = 300
		}
		dur := time.Duration(input.Seconds * float64(time.Second))
		select {
		case <-time.After(dur):
			return fmt.Sprintf("Slept %.1fs.", input.Seconds), nil
		case <-ctx.Done():
			return "", ctx.Err()
		}
	},
}
