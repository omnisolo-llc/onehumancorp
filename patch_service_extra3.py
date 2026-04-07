import re

with open("srcs/server/orchestration/service_extra_test.go", "r") as f:
    content = f.read()

# the test manually acquires the global lock but updates the shard map which leads to race condition/deadlock, let's fix it by acquiring the shard lock
content = content.replace(
"""	hub.mu.Lock()
	ch := make(chan struct{}, 1)
	hub.getShard("swe-1").subs["swe-1"] = append(hub.getShard("swe-1").subs["swe-1"], ch)
	hub.mu.Unlock()""",
"""	shard := hub.getShard("swe-1")
	shard.mu.Lock()
	ch := make(chan struct{}, 1)
	shard.subs["swe-1"] = append(shard.subs["swe-1"], ch)
	shard.mu.Unlock()""")

with open("srcs/server/orchestration/service_extra_test.go", "w") as f:
    f.write(content)
