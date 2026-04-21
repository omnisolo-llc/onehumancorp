package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/lib/perf"
)

// agentMailbox is a per-agent inbox for inter-agent messaging.
// Mirrors CC-Source's SendMessageTool which routes messages to a task's pendingMessages queue.
var agentMailbox = &MailboxRegistry{
	mailbox: perf.NewShardedMailbox(64),
}

// MailboxRegistry holds per-agent mailboxes.
type MailboxRegistry struct {
	mailbox *perf.ShardedMailbox
}

// Push appends a message to the named agent's inbox.
func (r *MailboxRegistry) Push(agentID, msg string) {
	m := perf.Message{
		ID:        "msg-" + agentID,
		Recipient: agentID,
		Payload:   []byte(msg),
	}
	_ = r.mailbox.Send(m)
}

// Drain returns and clears all pending messages for agentID.
func (r *MailboxRegistry) Drain(agentID string) []string {
	msgs := r.mailbox.Read(agentID)
	if len(msgs) == 0 {
		return nil
	}
	out := make([]string, len(msgs))
	for i, m := range msgs {
		out[i] = string(m.Payload)
	}
	return out
}

// SendMessageTool sends a message to another agent or to stdout.
// When "to" is provided it routes via the MailboxRegistry.
// Otherwise it prints to stdout (for user-facing messages).
// Mirrors CC-Source's SendMessageTool + intra-process queueing.
var SendMessageTool = Tool{
	Name: "SendMessage",
	Description: "Send a message to the user or to another agent. " +
		"When 'to' is provided, the message is delivered to that agent's inbox. " +
		"When 'to' is omitted, the message is printed to the user.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"message": {
				"type": "string",
				"description": "The message to send."
			},
			"to": {
				"type": "string",
				"description": "Optional agent ID or name to route the message to."
			}
		},
		"required": ["message"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Message string `json:"message"`
			To      string `json:"to"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}
		if input.Message == "" {
			return "", fmt.Errorf("SendMessage: message is required")
		}

		if input.To != "" {
			// Route to agent mailbox.
			agentMailbox.Push(input.To, input.Message)
			return fmt.Sprintf("Message delivered to agent %q.", input.To), nil
		}

		// Print to stdout for user-facing messages.
		fmt.Fprintf(os.Stdout, "\n=== MESSAGE TO USER ===\n%s\n=======================\n", input.Message)
		return "Message sent.", nil
	},
}