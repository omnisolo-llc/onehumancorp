package builtin

// EstimateTokens provides a rough estimation of token count for Claude/OpenAI
func EstimateTokens(text string) int {
	// A very rough heuristic: ~4 characters per token for English text
	return len(text) / 4
}

// EstimateMessagesTokens estimates the total token count for a list of messages
func EstimateMessagesTokens(messages []Message) int {
	total := 0
	for _, m := range messages {
		total += EstimateTokens(m.Content)
		for _, tc := range m.ToolCalls {
			total += EstimateTokens(tc.Name)
			total += EstimateTokens(string(tc.Arguments))
		}
		for _, tr := range m.ToolResults {
			total += EstimateTokens(tr.Content)
			total += EstimateTokens(tr.Error)
		}
	}
	return total
}

// CompactMessages removes the oldest non-system messages to fit within the threshold
func CompactMessages(messages []Message, threshold int) []Message {
	currentTokens := EstimateMessagesTokens(messages)
	if currentTokens <= threshold {
		return messages
	}

	var compacted []Message
	// Always keep the system prompt if present
	startIndex := 0
	for i, m := range messages {
		if m.Role == RoleSystem {
			compacted = append(compacted, m)
			startIndex = i + 1
		} else {
			break
		}
	}

	// Calculate how many tokens we need to free
	tokensToFree := currentTokens - threshold
	freedTokens := 0

	// Find the index to start keeping messages
	keepIndex := startIndex
	for i := startIndex; i < len(messages); i++ {
		if freedTokens >= tokensToFree {
			break
		}
		// If it's a user message and the next is an assistant message with tool calls,
		// we probably want to drop them together, but for simplicity, we just drop
		// oldest messages until we've freed enough tokens.
		// A more robust implementation would keep user/assistant pairs intact.

		freedTokens += EstimateTokens(messages[i].Content)
		for _, tc := range messages[i].ToolCalls {
			freedTokens += EstimateTokens(tc.Name)
			freedTokens += EstimateTokens(string(tc.Arguments))
		}
		for _, tr := range messages[i].ToolResults {
			freedTokens += EstimateTokens(tr.Content)
			freedTokens += EstimateTokens(tr.Error)
		}
		keepIndex = i + 1
	}

	// Add an indication that messages were compacted
	if keepIndex > startIndex {
		compacted = append(compacted, Message{
			Role: RoleUser,
			Content: "[System: Older messages were compacted to save context window]",
		})
	}

	compacted = append(compacted, messages[keepIndex:]...)
	return compacted
}
