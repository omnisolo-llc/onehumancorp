import re

with open("srcs/server/orchestration/service.go", "r") as f:
    content = f.read()

# Replace RegisterAgent
content = content.replace(
"""	h.mu.Lock()
	defer h.mu.Unlock()

	h.agents[agent.ID] = agent

	if h.sipDB != nil {
		go func(a Agent) {
			_ = h.sipDB.Heartbeat(context.Background(), a.ID, a.Role, string(a.Status))
		}(agent)
	}""",
"""	shard := h.getShard(agent.ID)
	shard.mu.Lock()
	defer shard.mu.Unlock()

	shard.agents[agent.ID] = agent

	h.mu.RLock()
	sipDB := h.sipDB
	h.mu.RUnlock()

	if sipDB != nil {
		go func(a Agent) {
			_ = sipDB.Heartbeat(context.Background(), a.ID, a.Role, string(a.Status))
		}(agent)
	}"""
)

# Replace FireAgent
content = content.replace(
"""	h.mu.Lock()
	defer h.mu.Unlock()

	delete(h.agents, id)
	delete(h.inbox, id)""",
"""	shard := h.getShard(id)
	shard.mu.Lock()
	defer shard.mu.Unlock()

	delete(shard.agents, id)
	delete(shard.inbox, id)"""
)

# Replace Subscribe
content = content.replace(
"""func (h *Hub) Subscribe(agentID string) (<-chan struct{}, func()) {
	h.mu.Lock()
	defer h.mu.Unlock()

	ch := make(chan struct{}, 1)
	h.subs[agentID] = append(h.subs[agentID], ch)

	unsubscribe := func() {
		h.mu.Lock()
		defer h.mu.Unlock()
		subs := h.subs[agentID]
		for i, sub := range subs {
			if sub == ch {
				// Prevent memory leak from lingering reference in underlying array
				subs[i] = nil
				h.subs[agentID] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
		if len(h.subs[agentID]) == 0 {
			delete(h.subs, agentID)
		}
	}

	return ch, unsubscribe
}""",
"""func (h *Hub) Subscribe(agentID string) (<-chan struct{}, func()) {
	shard := h.getShard(agentID)
	shard.mu.Lock()
	defer shard.mu.Unlock()

	ch := make(chan struct{}, 1)
	shard.subs[agentID] = append(shard.subs[agentID], ch)

	unsubscribe := func() {
		shard.mu.Lock()
		defer shard.mu.Unlock()
		subs := shard.subs[agentID]
		for i, sub := range subs {
			if sub == ch {
				// Prevent memory leak from lingering reference in underlying array
				subs[i] = nil
				shard.subs[agentID] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
		if len(shard.subs[agentID]) == 0 {
			delete(shard.subs, agentID)
		}
	}

	return ch, unsubscribe
}"""
)

# Replace OpenMeeting
content = content.replace(
"""	h.mu.Lock()
	defer h.mu.Unlock()

	meeting := MeetingRoom{ID: id, Participants: append([]string(nil), participants...)}
	h.meetings[id] = meeting
	for _, participant := range participants {
		agent := h.agents[participant]
		agent.Status = StatusInMeeting
		h.agents[participant] = agent
	}""",
"""	h.mu.Lock()
	defer h.mu.Unlock()

	meeting := MeetingRoom{ID: id, Participants: append([]string(nil), participants...)}
	h.meetings[id] = meeting
	for _, participant := range participants {
		shard := h.getShard(participant)
		shard.mu.Lock()
		agent := shard.agents[participant]
		agent.Status = StatusInMeeting
		shard.agents[participant] = agent
		shard.mu.Unlock()
	}"""
)

# Replace OpenMeetingWithAgenda
content = content.replace(
"""	h.mu.Lock()
	defer h.mu.Unlock()

	meeting := MeetingRoom{ID: id, Agenda: agenda, Participants: append([]string(nil), participants...)}
	h.meetings[id] = meeting
	for _, participant := range participants {
		agent := h.agents[participant]
		agent.Status = StatusInMeeting
		h.agents[participant] = agent
	}""",
"""	h.mu.Lock()
	defer h.mu.Unlock()

	meeting := MeetingRoom{ID: id, Agenda: agenda, Participants: append([]string(nil), participants...)}
	h.meetings[id] = meeting
	for _, participant := range participants {
		shard := h.getShard(participant)
		shard.mu.Lock()
		agent := shard.agents[participant]
		agent.Status = StatusInMeeting
		shard.agents[participant] = agent
		shard.mu.Unlock()
	}"""
)

# Replace Agents() method
content = content.replace(
"""	h.mu.RLock()
	agents := make([]Agent, 0, len(h.agents))
	for _, agent := range h.agents {
		agents = append(agents, agent)
	}
	h.mu.RUnlock()""",
"""	var agents []Agent
	for _, shard := range h.shards {
		shard.mu.RLock()
		for _, agent := range shard.agents {
			agents = append(agents, agent)
		}
		shard.mu.RUnlock()
	}"""
)

with open("srcs/server/orchestration/service.go", "w") as f:
    f.write(content)
