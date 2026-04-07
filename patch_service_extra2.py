import re

with open("srcs/server/orchestration/service_extra_test.go", "r") as f:
    content = f.read()

content = content.replace("hub.getShard(\"a\").subs[\"swe-1\"]", "hub.getShard(\"swe-1\").subs[\"swe-1\"]")

with open("srcs/server/orchestration/service_extra_test.go", "w") as f:
    f.write(content)

with open("srcs/server/orchestration/service.go", "r") as f:
    content = f.read()

content = content.replace(
"""	shard := h.getShard(agent.ID)
	shard.mu.Lock()
	defer shard.mu.Unlock()

	shard.agents[agent.ID] = agent

	h.mu.RLock()
	sipDB := h.sipDB
	h.mu.RUnlock()""",
"""	h.mu.RLock()
	sipDB := h.sipDB
	h.mu.RUnlock()

	shard := h.getShard(agent.ID)
	shard.mu.Lock()
	shard.agents[agent.ID] = agent
	shard.mu.Unlock()""")

with open("srcs/server/orchestration/service.go", "w") as f:
    f.write(content)
