import re

with open("srcs/server/orchestration/service.go", "r") as f:
    content = f.read()

# Replace Hub.Publish
old_publish = """// Publish validates and routes a message to a direct recipient, a meeting room, or both.
//
//   - message: Message; The event payload containing routing headers and content.
//
// Accepts parameters: h *Hub (No Constraints).
// Returns Publish(message Message) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (h *Hub) Publish(message Message) error {
	if h.repo != nil {
		return h.publishRepository(message)
	}

	h.mu.Lock()
	sender, senderOk := h.agents[message.FromAgent]
	if !senderOk {
		h.mu.Unlock()
		return errors.New("sender agent is not registered")
	}

	if message.ToAgent != "" {
		if _, ok := h.agents[message.ToAgent]; !ok {
			h.mu.Unlock()
			return errors.New("recipient agent is not registered")
		}
	}

	if message.MeetingID != "" {
		if _, ok := h.meetings[message.MeetingID]; !ok {
			h.mu.Unlock()
			return errors.New("meeting room is not registered")
		}
	}

	if message.ToAgent != "" {
		inbox := h.inbox[message.ToAgent]
		if cap(inbox) == 0 {
			inbox = getMessageSlice()
		}

		h.inbox[message.ToAgent] = append(inbox, message)

		if subs, ok := h.subs[message.ToAgent]; ok {
			for _, sub := range subs {
				select {
				case sub <- struct{}{}:
				default:
				}
			}
		}
	}

	if message.MeetingID != "" {
		meeting, ok := h.meetings[message.MeetingID]
		if !ok {
			h.mu.Unlock()
			return errors.New("meeting room is not registered")
		}

		if cap(meeting.Transcript) == 0 {
			meeting.Transcript = getMessageSlice()
		}
		meeting.Transcript = append(meeting.Transcript, message)

		// ⚡ BOLT: [Aggressive AI Context Summarization] - Randomized Selection from Top 5
		if len(meeting.Transcript) > 10 && h.minimaxAPIKey != "" {
			minimaxKey := h.minimaxAPIKey
			// Copy transcript for async processing to avoid retaining the backing array long-term
			// We only need the FromAgent and Content.
			type transcriptLine struct {
				Agent   string `json:"agent"`
				Content string `json:"content"`
			}
			var lines []transcriptLine
			for _, msg := range meeting.Transcript {
				lines = append(lines, transcriptLine{Agent: msg.FromAgent, Content: msg.Content}) // Redact in goroutine
			}

			go func(mID string, tLines []transcriptLine) {
				ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
				defer cancel()
				client := NewMinimaxClient(minimaxKey)

				// Redact PII in the goroutine to save main thread CPU
				for i, line := range tLines {
					tLines[i].Content = telemetry.RedactPII(line.Content)
				}
				jsonPayload, _ := json.Marshal(tLines)
				prompt := "Extract and summarize ONLY the exact parameters, architectural decisions, and required next steps from this transcript. Discard all conversational filler, pleasantries, and non-actionable text. Output MUST be an ultra-dense, bulleted technical brief optimized for minimal token footprint:\\n" + string(jsonPayload)

				summary, err := client.Reason(ctx, prompt)
				if err == nil && summary != "" {
					h.mu.Lock()
					if mtg, ok := h.meetings[mID]; ok {
						newTranscript := []Message{
							{
								ID:         "summary-" + time.Now().UTC().Format("20060102150405"),
								FromAgent:  "SYSTEM_SUMMARIZER",
								ToAgent:    "all",
								Type:       EventStatus,
								Content:    "[CONTEXT SUMMARIZED]: " + summary,
								MeetingID:  mID,
								OccurredAt: time.Now().UTC(),
							},
						}
						if len(mtg.Transcript) > 3 {
							newTranscript = append(newTranscript, mtg.Transcript[len(mtg.Transcript)-3:]...)
						} else {
							newTranscript = append(newTranscript, mtg.Transcript...)
						}
						mtg.Transcript = newTranscript
						h.meetings[mID] = mtg
					}
					h.mu.Unlock()
				} else {
					slog.Warn("context summarization failed", "meeting_id", mID, "error", err)
				}
			}(message.MeetingID, lines)
		}

		h.meetings[message.MeetingID] = meeting
		sender.Status = StatusInMeeting

		for _, participant := range meeting.Participants {
			if subs, ok := h.subs[participant]; ok {
				for _, sub := range subs {
					select {
					case sub <- struct{}{}:
					default:
					}
				}
			}
		}
	} else {
		sender.Status = StatusActive
	}

	h.agents[message.FromAgent] = sender
	centrifugeNode := h.centrifugeNode

	h.mu.Unlock()

	// ⚡ BOLT: [Parallel Execution] Async worker for telemetry, PII redaction and logging
	// Move heavy regex processing (redactPII) to a background goroutine to free up the Publisher thread
	go func(sID, sRole, mType, mContent string) {
		telemetry.RecordAgentApiCall(context.Background(), sID, sRole, "publish")
		if mType != EventStatus {
			telemetry.LogAgentExecution(context.Background(), sID, sRole, "publish", mType, telemetry.RedactPII(mContent))
		}
	}(sender.ID, sender.Role, message.Type, message.Content)

	if centrifugeNode != nil {
		go func(m Message, cn *CentrifugeNode) {
			if m.MeetingID != "" {
				cn.PublishMeetingMessage(m.MeetingID, m)
			}
			if m.ToAgent != "" && m.Type != "mesh:tasks" && m.Type != "mesh:coordination" && m.Type != "mesh:direct" {
				cn.PublishAgentNotification(m.ToAgent, m)
			}
			if m.Type == "mesh:tasks" {
				var payload map[string]interface{}
				if err := json.Unmarshal([]byte(m.Content), &payload); err == nil {
					cn.PublishTaskBroadcast(m.ID, payload)
				}
			} else if m.Type == "mesh:coordination" {
				cn.PublishCoordinationMessage(m)
			} else if m.Type == "mesh:direct" && m.ToAgent != "" {
				cn.PublishAgentNotification(m.ToAgent, m)
			}
		}(message, centrifugeNode)
	}

	return nil
}"""

new_publish = """// Publish validates and routes a message to a direct recipient, a meeting room, or both.
//
//   - message: Message; The event payload containing routing headers and content.
//
// Accepts parameters: h *Hub (No Constraints).
// Returns Publish(message Message) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (h *Hub) Publish(message Message) error {
	if h.repo != nil {
		return h.publishRepository(message)
	}

	senderShard := h.getShard(message.FromAgent)
	senderShard.mu.RLock()
	sender, senderOk := senderShard.agents[message.FromAgent]
	senderShard.mu.RUnlock()

	if !senderOk {
		return errors.New("sender agent is not registered")
	}

	if message.ToAgent != "" {
		recipientShard := h.getShard(message.ToAgent)
		recipientShard.mu.RLock()
		if _, ok := recipientShard.agents[message.ToAgent]; !ok {
			recipientShard.mu.RUnlock()
			return errors.New("recipient agent is not registered")
		}
		recipientShard.mu.RUnlock()
	}

	h.mu.RLock()
	if message.MeetingID != "" {
		if _, ok := h.meetings[message.MeetingID]; !ok {
			h.mu.RUnlock()
			return errors.New("meeting room is not registered")
		}
	}
	h.mu.RUnlock()

	if message.ToAgent != "" {
		recipientShard := h.getShard(message.ToAgent)
		recipientShard.mu.Lock()
		inbox := recipientShard.inbox[message.ToAgent]
		if cap(inbox) == 0 {
			inbox = getMessageSlice()
		}

		recipientShard.inbox[message.ToAgent] = append(inbox, message)

		if subs, ok := recipientShard.subs[message.ToAgent]; ok {
			for _, sub := range subs {
				select {
				case sub <- struct{}{}:
				default:
				}
			}
		}
		recipientShard.mu.Unlock()
	}

	if message.MeetingID != "" {
		h.mu.Lock()
		meeting, ok := h.meetings[message.MeetingID]
		if !ok {
			h.mu.Unlock()
			return errors.New("meeting room is not registered")
		}

		if cap(meeting.Transcript) == 0 {
			meeting.Transcript = getMessageSlice()
		}
		meeting.Transcript = append(meeting.Transcript, message)

		// ⚡ BOLT: [Aggressive AI Context Summarization] - Randomized Selection from Top 5
		if len(meeting.Transcript) > 10 && h.minimaxAPIKey != "" {
			minimaxKey := h.minimaxAPIKey
			// Copy transcript for async processing to avoid retaining the backing array long-term
			// We only need the FromAgent and Content.
			type transcriptLine struct {
				Agent   string `json:"agent"`
				Content string `json:"content"`
			}
			var lines []transcriptLine
			for _, msg := range meeting.Transcript {
				lines = append(lines, transcriptLine{Agent: msg.FromAgent, Content: msg.Content}) // Redact in goroutine
			}

			go func(mID string, tLines []transcriptLine) {
				ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
				defer cancel()
				client := NewMinimaxClient(minimaxKey)

				// Redact PII in the goroutine to save main thread CPU
				for i, line := range tLines {
					tLines[i].Content = telemetry.RedactPII(line.Content)
				}
				jsonPayload, _ := json.Marshal(tLines)
				prompt := "Extract and summarize ONLY the exact parameters, architectural decisions, and required next steps from this transcript. Discard all conversational filler, pleasantries, and non-actionable text. Output MUST be an ultra-dense, bulleted technical brief optimized for minimal token footprint:\\n" + string(jsonPayload)

				summary, err := client.Reason(ctx, prompt)
				if err == nil && summary != "" {
					h.mu.Lock()
					if mtg, ok := h.meetings[mID]; ok {
						newTranscript := []Message{
							{
								ID:         "summary-" + time.Now().UTC().Format("20060102150405"),
								FromAgent:  "SYSTEM_SUMMARIZER",
								ToAgent:    "all",
								Type:       EventStatus,
								Content:    "[CONTEXT SUMMARIZED]: " + summary,
								MeetingID:  mID,
								OccurredAt: time.Now().UTC(),
							},
						}
						if len(mtg.Transcript) > 3 {
							newTranscript = append(newTranscript, mtg.Transcript[len(mtg.Transcript)-3:]...)
						} else {
							newTranscript = append(newTranscript, mtg.Transcript...)
						}
						mtg.Transcript = newTranscript
						h.meetings[mID] = mtg
					}
					h.mu.Unlock()
				} else {
					slog.Warn("context summarization failed", "meeting_id", mID, "error", err)
				}
			}(message.MeetingID, lines)
		}

		h.meetings[message.MeetingID] = meeting
		h.mu.Unlock()

		sender.Status = StatusInMeeting

		for _, participant := range meeting.Participants {
			pShard := h.getShard(participant)
			pShard.mu.RLock()
			if subs, ok := pShard.subs[participant]; ok {
				for _, sub := range subs {
					select {
					case sub <- struct{}{}:
					default:
					}
				}
			}
			pShard.mu.RUnlock()
		}
	} else {
		sender.Status = StatusActive
	}

	senderShard.mu.Lock()
	senderShard.agents[message.FromAgent] = sender
	senderShard.mu.Unlock()

	h.mu.RLock()
	centrifugeNode := h.centrifugeNode
	h.mu.RUnlock()

	// ⚡ BOLT: [Parallel Execution] Async worker for telemetry, PII redaction and logging
	// Move heavy regex processing (redactPII) to a background goroutine to free up the Publisher thread
	go func(sID, sRole, mType, mContent string) {
		telemetry.RecordAgentApiCall(context.Background(), sID, sRole, "publish")
		if mType != EventStatus {
			telemetry.LogAgentExecution(context.Background(), sID, sRole, "publish", mType, telemetry.RedactPII(mContent))
		}
	}(sender.ID, sender.Role, message.Type, message.Content)

	if centrifugeNode != nil {
		go func(m Message, cn *CentrifugeNode) {
			if m.MeetingID != "" {
				cn.PublishMeetingMessage(m.MeetingID, m)
			}
			if m.ToAgent != "" && m.Type != "mesh:tasks" && m.Type != "mesh:coordination" && m.Type != "mesh:direct" {
				cn.PublishAgentNotification(m.ToAgent, m)
			}
			if m.Type == "mesh:tasks" {
				var payload map[string]interface{}
				if err := json.Unmarshal([]byte(m.Content), &payload); err == nil {
					cn.PublishTaskBroadcast(m.ID, payload)
				}
			} else if m.Type == "mesh:coordination" {
				cn.PublishCoordinationMessage(m)
			} else if m.Type == "mesh:direct" && m.ToAgent != "" {
				cn.PublishAgentNotification(m.ToAgent, m)
			}
		}(message, centrifugeNode)
	}

	return nil
}"""

# Also fix notifyRealtimeTargets
old_notify = """func (h *Hub) notifyRealtimeTargets(message Message, meetingParticipants []string) *CentrifugeNode {
	h.mu.RLock()
	defer h.mu.RUnlock()

	if message.ToAgent != "" {
		if subs, ok := h.subs[message.ToAgent]; ok {
			for _, sub := range subs {
				select {
				case sub <- struct{}{}:
				default:
				}
			}
		}
	}

	for _, participant := range meetingParticipants {
		if subs, ok := h.subs[participant]; ok {
			for _, sub := range subs {
				select {
				case sub <- struct{}{}:
				default:
				}
			}
		}
	}

	return h.centrifugeNode
}"""

new_notify = """func (h *Hub) notifyRealtimeTargets(message Message, meetingParticipants []string) *CentrifugeNode {
	if message.ToAgent != "" {
		recipientShard := h.getShard(message.ToAgent)
		recipientShard.mu.RLock()
		if subs, ok := recipientShard.subs[message.ToAgent]; ok {
			for _, sub := range subs {
				select {
				case sub <- struct{}{}:
				default:
				}
			}
		}
		recipientShard.mu.RUnlock()
	}

	for _, participant := range meetingParticipants {
		pShard := h.getShard(participant)
		pShard.mu.RLock()
		if subs, ok := pShard.subs[participant]; ok {
			for _, sub := range subs {
				select {
				case sub <- struct{}{}:
				default:
				}
			}
		}
		pShard.mu.RUnlock()
	}

	h.mu.RLock()
	defer h.mu.RUnlock()
	return h.centrifugeNode
}"""

if old_publish in content:
    content = content.replace(old_publish, new_publish)
else:
    print("Failed to find old publish method")

if old_notify in content:
    content = content.replace(old_notify, new_notify)
else:
    print("Failed to find old notify method")

with open("srcs/server/orchestration/service.go", "w") as f:
    f.write(content)
