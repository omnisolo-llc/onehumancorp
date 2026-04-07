import re

with open("srcs/server/orchestration/service.go", "r") as f:
    content = f.read()

# Fix lock inversion in RegisterAgent
content = content.replace(
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
	}""",
"""	h.mu.RLock()
	sipDB := h.sipDB
	h.mu.RUnlock()

	shard := h.getShard(agent.ID)
	shard.mu.Lock()
	shard.agents[agent.ID] = agent
	shard.mu.Unlock()

	if sipDB != nil {
		go func(a Agent) {
			_ = sipDB.Heartbeat(context.Background(), a.ID, a.Role, string(a.Status))
		}(agent)
	}"""
)

with open("srcs/server/orchestration/service.go", "w") as f:
    f.write(content)

# Fix self-deadlock in delegation.go
with open("srcs/server/orchestration/delegation.go", "r") as f:
    content = f.read()

# Remove the lock around RegisterAgent inside delegation.go
# because RegisterAgent handles its own locking, and h.mu is a global lock
content = content.replace(
"""	s.hub.mu.Unlock()

	if _, exists := s.hub.Agent(subAgent.ID); !exists {
		s.hub.RegisterAgent(subAgent)
	}""",
"""	s.hub.mu.Unlock()

	if _, exists := s.hub.Agent(subAgent.ID); !exists {
		s.hub.RegisterAgent(subAgent)
	}""" # wait, I already removed it in a previous patch? Let me check carefully.
)
with open("srcs/server/orchestration/delegation.go", "w") as f:
    f.write(content)
