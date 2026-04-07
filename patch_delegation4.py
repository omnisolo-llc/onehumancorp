import re

# Fix eventlog_test.go again
with open("srcs/server/orchestration/eventlog_test.go", "r") as f:
    content = f.read()

content = content.replace(
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

# Actually, the problem is in eventlog_test.go, let's just see what it actually is:
with open("srcs/server/orchestration/eventlog_test.go", "r") as f:
    print(f.read())
