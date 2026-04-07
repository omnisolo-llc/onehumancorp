import re

# Fix eventlog_test.go
with open("srcs/server/orchestration/eventlog_test.go", "r") as f:
    content = f.read()

content = content.replace(
"""	h := &Hub{
		agents:        map[string]Agent{},
		inbox:         map[string][]Message{},
		meetings:      map[string]MeetingRoom{},
		subs:          map[string][]chan struct{}{},
		tokenTrackers: map[string]struct{}{},
		autoCorTrack:  map[string]struct{}{},
		eventLogChan:  make(chan interface{}, 100),
		ctx:           ctx,
		cancel:        cancel,
	}""",
"""	h := &Hub{
		shards:        make([]*HubShard, 16),
		meetings:      map[string]MeetingRoom{},
		tokenTrackers: map[string]struct{}{},
		autoCorTrack:  map[string]struct{}{},
		eventLogChan:  make(chan interface{}, 100),
		ctx:           ctx,
		cancel:        cancel,
	}
	for i := 0; i < 16; i++ {
		h.shards[i] = &HubShard{
			agents: map[string]Agent{},
			inbox:  map[string][]Message{},
			subs:   map[string][]chan struct{}{},
		}
	}"""
)

with open("srcs/server/orchestration/eventlog_test.go", "w") as f:
    f.write(content)

# Fix service_extra_test.go
with open("srcs/server/orchestration/service_extra_test.go", "r") as f:
    content = f.read()

content = content.replace("hub.subs[", "hub.getShard(\\\"a\\\").subs[")
content = content.replace("hub.getShard(\\\"a\\\").subs[\\\"a\\\"]", "hub.getShard(\\\"a\\\").subs[\\\"a\\\"]")

with open("srcs/server/orchestration/service_extra_test.go", "w") as f:
    f.write(content)

# Fix delegation_test.go again
with open("srcs/server/orchestration/delegation_test.go", "r") as f:
    content = f.read()
content = content.replace("hub.FireAgent(\"sender-fail\")", "/* fired */")
content = content.replace("delete(hub.Agents(), \\\"sender-fail\\\")", "hub.FireAgent(\\\"sender-fail\\\")")
with open("srcs/server/orchestration/delegation_test.go", "w") as f:
    f.write(content)
