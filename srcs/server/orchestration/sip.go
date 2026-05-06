package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

// SIPDB extends TaskStore with Omni-Context Routing capabilities.
type SIPDB struct {
	TaskStore
	ContextRoot string
}

func NewSIPDB(store TaskStore, contextRoot string) *SIPDB {
	return &SIPDB{
		TaskStore:   store,
		ContextRoot: contextRoot,
	}
}

func (s *SIPDB) DelegateMission(ctx context.Context, task *SharedTask) error {
	var groundingContent []byte

	if s.ContextRoot != "" {
		agentsFile := filepath.Join(s.ContextRoot, "AGENTS.md")
		claudeFile := filepath.Join(s.ContextRoot, "CLAUDE.md")

		if content, errAgents := os.ReadFile(agentsFile); errAgents == nil {
			groundingContent = content
		} else if content, errClaude := os.ReadFile(claudeFile); errClaude == nil {
			groundingContent = content
		}
	}

	if len(groundingContent) > 0 {
		groundingStr := fmt.Sprintf("\n\n[SYSTEM GROUNDING]:\n%s", string(groundingContent))

		if task.Payload != nil && len(*task.Payload) > 0 {
			// Try to parse as JSON string
			var str string
			if err := json.Unmarshal(*task.Payload, &str); err == nil {
				str += groundingStr
				newRaw, _ := json.Marshal(str)
				raw := json.RawMessage(newRaw)
				task.Payload = &raw
			} else {
				// Try to parse as JSON object
				var obj map[string]interface{}
				if err := json.Unmarshal(*task.Payload, &obj); err == nil {
					if content, ok := obj["content"].(string); ok {
						obj["content"] = content + groundingStr
					} else {
						obj["content"] = groundingStr
					}
					newRaw, _ := json.Marshal(obj)
					raw := json.RawMessage(newRaw)
					task.Payload = &raw
				} else {
					// Fallback to raw string append
					newText := string(*task.Payload) + groundingStr
					raw := json.RawMessage(newText)
					task.Payload = &raw
				}
			}
		} else {
			newRaw, _ := json.Marshal(groundingStr)
			raw := json.RawMessage(newRaw)
			task.Payload = &raw
		}
	}

	return s.CreateTask(ctx, task)
}
