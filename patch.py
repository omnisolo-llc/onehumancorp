import re

with open("srcs/server/orchestration/service.go", "r") as f:
    content = f.read()

# Replace Hub struct
content = content.replace(
"""type Hub struct {
	mu             sync.RWMutex
	agents         map[string]Agent
	inbox          map[string][]Message
	meetings       map[string]MeetingRoom
	minimaxAPIKey  string
	subs           map[string][]chan struct{}
	sipDB          *SIPDB
	tokenTrackers  map[string]struct{}
	GetTokenUsage  func(ctx context.Context) map[string]int64
	autoCorTrack   map[string]struct{}
	eventLogChan   chan interface{}
	repo           HubRepository
	scheduler      *scheduler.Scheduler
	settingsStore  *settings.Store
	centrifugeNode *CentrifugeNode
	storage        storage.Provider
	ctx            context.Context
	cancel         context.CancelFunc
	taskManager    *TaskManager
}""",
"""// HubShard contains sharded state to prevent global lock contention during message routing.
type HubShard struct {
	mu     sync.RWMutex
	agents map[string]Agent
	inbox  map[string][]Message
	subs   map[string][]chan struct{}
}

// Hub acts as the central, thread-safe asynchronous message broker and state registry for all active agents and meeting rooms.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Hub struct {
	mu             sync.RWMutex
	shards         []*HubShard // Parallel Execution Hooks: Sharded Agent state
	meetings       map[string]MeetingRoom
	minimaxAPIKey  string
	sipDB          *SIPDB
	tokenTrackers  map[string]struct{}
	GetTokenUsage  func(ctx context.Context) map[string]int64
	autoCorTrack   map[string]struct{}
	eventLogChan   chan interface{}
	repo           HubRepository
	scheduler      *scheduler.Scheduler
	settingsStore  *settings.Store
	centrifugeNode *CentrifugeNode
	storage        storage.Provider
	ctx            context.Context
	cancel         context.CancelFunc
	taskManager    *TaskManager
}

func (h *Hub) getShard(key string) *HubShard {
	var hash uint32
	for i := 0; i < len(key); i++ {
		hash = hash*31 + uint32(key[i])
	}
	return h.shards[hash%uint32(len(h.shards))]
}"""
)

# Replace newHub
content = content.replace(
"""	h := &Hub{
		agents:        map[string]Agent{},
		inbox:         map[string][]Message{},
		meetings:      map[string]MeetingRoom{},
		subs:          map[string][]chan struct{}{},
		tokenTrackers: map[string]struct{}{},
		autoCorTrack:  map[string]struct{}{},
		eventLogChan:  make(chan interface{}, 100),
		repo:          repo,
		scheduler:     sched,
		settingsStore: settings.NewStore(),
		ctx:           ctx,
		cancel:        cancel,
	}""",
"""	h := &Hub{
		shards:        make([]*HubShard, 16),
		meetings:      map[string]MeetingRoom{},
		tokenTrackers: map[string]struct{}{},
		autoCorTrack:  map[string]struct{}{},
		eventLogChan:  make(chan interface{}, 100),
		repo:          repo,
		scheduler:     sched,
		settingsStore: settings.NewStore(),
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

# Replace Agent retrieval
content = content.replace(
"""	h.mu.RLock()
	defer h.mu.RUnlock()

	agent, ok := h.agents[id]
	return agent, ok""",
"""	shard := h.getShard(id)
	shard.mu.RLock()
	defer shard.mu.RUnlock()

	agent, ok := shard.agents[id]
	return agent, ok"""
)

# Replace Inbox
content = content.replace(
"""	h.mu.Lock()
	defer h.mu.Unlock()

	inbox := h.inbox[agentID]
	if len(inbox) == 0 {
		return nil
	}

	// ⚡ BOLT: [O(1) Inbox draining instead of O(N) slice copy] - Randomized Selection from Top 5
	// To prevent memory leak of map keys over time, delete the key entirely.
	delete(h.inbox, agentID)
	return inbox""",
"""	shard := h.getShard(agentID)
	shard.mu.Lock()
	defer shard.mu.Unlock()

	inbox := shard.inbox[agentID]
	if len(inbox) == 0 {
		return nil
	}

	// ⚡ BOLT: [O(1) Inbox draining instead of O(N) slice copy] - Randomized Selection from Top 5
	// To prevent memory leak of map keys over time, delete the key entirely.
	delete(shard.inbox, agentID)
	return inbox"""
)

with open("srcs/server/orchestration/service.go", "w") as f:
    f.write(content)
