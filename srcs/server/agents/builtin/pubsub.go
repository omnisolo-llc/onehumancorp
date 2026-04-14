package builtin

import (
	"sync"
	"time"
)

// SubagentEventType identifies the kind of subagent lifecycle event.
//
// These values mirror the ohc.agent.service.SubagentLifecycleEvent.EventType
// protobuf enum defined in agent_service.proto.
type SubagentEventType int32

const (
	SubagentEventUnspecified SubagentEventType = 0
	// SubagentEventSpawned is published immediately after a sub-agent is started.
	SubagentEventSpawned SubagentEventType = 1
	// SubagentEventHeartbeat is published periodically while the task runs.
	SubagentEventHeartbeat SubagentEventType = 2
	// SubagentEventCompleted is published when the sub-agent finishes successfully.
	SubagentEventCompleted SubagentEventType = 3
	// SubagentEventFailed is published when the sub-agent fails.
	SubagentEventFailed SubagentEventType = 4
	// SubagentEventKilled is published when the sub-agent is forcefully stopped.
	SubagentEventKilled SubagentEventType = 5
)

// SubagentHeartbeat carries liveness and progress info for a running sub-agent.
//
// Mirrors the ohc.agent.service.SubagentHeartbeat protobuf message.
type SubagentHeartbeat struct {
	TaskID       string `json:"task_id"`        // field 1
	TimestampMs  int64  `json:"timestamp_ms"`   // field 2
	Status       string `json:"status"`         // field 3
	TokenCount   int64  `json:"token_count"`    // field 4
	ToolUseCount int64  `json:"tool_use_count"` // field 5
	LastActivity string `json:"last_activity"`  // field 6
}

// SubagentLifecycleEvent is the unified event published on the SubagentBus.
//
// Mirrors the ohc.agent.service.SubagentLifecycleEvent protobuf message.
type SubagentLifecycleEvent struct {
	EventType    SubagentEventType `json:"event_type"`              // field 1
	TaskID       string            `json:"task_id"`                 // field 2
	ParentTaskID string            `json:"parent_task_id,omitempty"` // field 3
	TimestampMs  int64             `json:"timestamp_ms"`            // field 4

	// Only one of these is set per event (mirrors a proto oneof).
	Heartbeat    *SubagentHeartbeat `json:"heartbeat,omitempty"`    // field 5
	Notification *TaskNotification  `json:"notification,omitempty"` // field 6
}

// SubagentBus is an in-process pub/sub bus for subagent lifecycle events.
//
// Publishers call Publish to broadcast an event.
// Subscribers call Subscribe to receive a channel of events for a specific task.
// The bus is designed for single-host, in-process use; cross-process delivery
// is handled by the Hub's pub/sub mechanism via SendMessageTool.
type SubagentBus struct {
	mu   sync.RWMutex
	subs map[string][]chan SubagentLifecycleEvent
}

// NewSubagentBus creates a new in-process event bus.
func NewSubagentBus() *SubagentBus {
	return &SubagentBus{subs: make(map[string][]chan SubagentLifecycleEvent)}
}

// Subscribe returns a channel that receives lifecycle events for taskID, and
// an unsubscribe function.  The channel is buffered (capacity 32) to avoid
// blocking the publisher.  Callers must call the unsubscribe function when done
// to release resources.
func (b *SubagentBus) Subscribe(taskID string) (<-chan SubagentLifecycleEvent, func()) {
	ch := make(chan SubagentLifecycleEvent, 32)
	b.mu.Lock()
	b.subs[taskID] = append(b.subs[taskID], ch)
	b.mu.Unlock()

	unsub := func() {
		b.mu.Lock()
		defer b.mu.Unlock()
		list := b.subs[taskID]
		filtered := list[:0]
		for _, existing := range list {
			if existing != ch {
				filtered = append(filtered, existing)
			}
		}
		if len(filtered) == 0 {
			delete(b.subs, taskID)
		} else {
			b.subs[taskID] = filtered
		}
		// Drain and close the channel so the subscriber goroutine can exit.
		close(ch)
		for range ch {
		}
	}
	return ch, unsub
}

// Publish broadcasts an event to all current subscribers for the event's task.
// It is non-blocking: if a subscriber's channel is full the event is dropped
// for that subscriber only.
func (b *SubagentBus) Publish(evt SubagentLifecycleEvent) {
	if evt.TimestampMs == 0 {
		evt.TimestampMs = time.Now().UnixMilli()
	}
	b.mu.RLock()
	list := b.subs[evt.TaskID]
	if len(list) == 0 {
		b.mu.RUnlock()
		return
	}
	// Copy so we can release the lock before sending.
	snapshot := make([]chan SubagentLifecycleEvent, len(list))
	copy(snapshot, list)
	b.mu.RUnlock()

	for _, ch := range snapshot {
		select {
		case ch <- evt:
		default:
			// Subscriber is slow; drop this event rather than blocking the publisher.
		}
	}
}

// globalSubagentBus is the process-level subagent lifecycle event bus.
// All AgentTool invocations publish and subscribe here.
var globalSubagentBus = NewSubagentBus()
