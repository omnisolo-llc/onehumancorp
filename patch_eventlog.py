import re

with open("srcs/server/orchestration/eventlog_test.go", "r") as f:
    content = f.read()

# Replace the struct init
content = content.replace(
"""	hub2 := &Hub{
		agents:        make(map[string]Agent),
		inbox:         make(map[string][]Message),
		meetings:      make(map[string]MeetingRoom),
		subs:          make(map[string][]chan struct{}),
		tokenTrackers: make(map[string]struct{}),
		autoCorTrack:  make(map[string]struct{}),
		eventLogChan:  make(chan interface{}, 1000),
	}""",
"""	hub2 := &Hub{
		shards:        make([]*HubShard, 16),
		meetings:      make(map[string]MeetingRoom),
		tokenTrackers: make(map[string]struct{}),
		autoCorTrack:  make(map[string]struct{}),
		eventLogChan:  make(chan interface{}, 1000),
	}
	for i := 0; i < 16; i++ {
		hub2.shards[i] = &HubShard{
			agents: make(map[string]Agent),
			inbox:  make(map[string][]Message),
			subs:   make(map[string][]chan struct{}),
		}
	}"""
)

with open("srcs/server/orchestration/eventlog_test.go", "w") as f:
    f.write(content)

with open("srcs/server/orchestration/delegation.go", "r") as f:
    content = f.read()

content = content.replace(
"""	currentAgents := len(s.hub.agents)

	// VRAM Quota Enforcement: Hard limit at 10 active agents across the hub
	if currentAgents >= 10 {
		s.hub.mu.Unlock()
		return nil, status.Errorf(codes.ResourceExhausted, "VRAM quota limit exceeded, cannot spawn sub-agent")
	}""",
"""	currentAgents := len(s.hub.Agents())

	// VRAM Quota Enforcement: Hard limit at 10 active agents across the hub
	if currentAgents >= 10 {
		s.hub.mu.Unlock()
		return nil, status.Errorf(codes.ResourceExhausted, "VRAM quota limit exceeded, cannot spawn sub-agent")
	}"""
)

content = content.replace(
"""	if _, exists := /* skip directly modifying hub.agents */; !exists {
		agent = subAgent
	}
	s.hub.mu.Unlock()""",
"""	s.hub.mu.Unlock()

	if _, exists := s.hub.Agent(subAgent.ID); !exists {
		s.hub.RegisterAgent(subAgent)
	}"""
)

with open("srcs/server/orchestration/delegation.go", "w") as f:
    f.write(content)
