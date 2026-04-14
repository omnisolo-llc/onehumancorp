package builtin

import (
	"testing"
	"time"
)

func TestSubagentBusPublishSubscribe(t *testing.T) {
	bus := NewSubagentBus()

	sub, unsub := bus.Subscribe("task-1")
	defer unsub()

	evt := SubagentLifecycleEvent{
		EventType: SubagentEventSpawned,
		TaskID:    "task-1",
	}
	bus.Publish(evt)

	select {
	case received := <-sub:
		if received.EventType != SubagentEventSpawned {
			t.Errorf("expected Spawned, got %v", received.EventType)
		}
		if received.TaskID != "task-1" {
			t.Errorf("expected task-1, got %q", received.TaskID)
		}
		if received.TimestampMs == 0 {
			t.Error("TimestampMs should be set by bus")
		}
	case <-time.After(time.Second):
		t.Fatal("timed out waiting for event")
	}
}

func TestSubagentBusUnsubscribe(t *testing.T) {
	bus := NewSubagentBus()

	sub, unsub := bus.Subscribe("task-2")
	unsub() // unsubscribe before publishing

	bus.Publish(SubagentLifecycleEvent{EventType: SubagentEventSpawned, TaskID: "task-2"})

	// After unsubscribe, the channel should be closed and no new events received.
	select {
	case _, ok := <-sub:
		if ok {
			t.Error("should not receive event after unsubscribe")
		}
		// channel closed - correct
	case <-time.After(100 * time.Millisecond):
		t.Error("expected closed channel after unsubscribe")
	}
}

func TestSubagentBusNoSubscribers(t *testing.T) {
	bus := NewSubagentBus()
	// Should not panic when there are no subscribers.
	bus.Publish(SubagentLifecycleEvent{EventType: SubagentEventCompleted, TaskID: "task-3"})
}

func TestSubagentBusMultipleSubscribers(t *testing.T) {
	bus := NewSubagentBus()

	sub1, unsub1 := bus.Subscribe("task-4")
	sub2, unsub2 := bus.Subscribe("task-4")
	defer unsub1()
	defer unsub2()

	bus.Publish(SubagentLifecycleEvent{EventType: SubagentEventCompleted, TaskID: "task-4"})

	for i, sub := range []<-chan SubagentLifecycleEvent{sub1, sub2} {
		select {
		case evt := <-sub:
			if evt.EventType != SubagentEventCompleted {
				t.Errorf("sub%d: expected Completed", i+1)
			}
		case <-time.After(time.Second):
			t.Fatalf("sub%d: timed out", i+1)
		}
	}
}

func TestSubagentBusIsolatesTaskIDs(t *testing.T) {
	bus := NewSubagentBus()

	subA, unsubA := bus.Subscribe("task-A")
	defer unsubA()

	// Publish to a different task — subA should NOT receive it.
	bus.Publish(SubagentLifecycleEvent{EventType: SubagentEventSpawned, TaskID: "task-B"})

	select {
	case evt := <-subA:
		t.Errorf("received event for wrong task: %v", evt)
	case <-time.After(50 * time.Millisecond):
		// Correct: no event for task-A
	}
}

func TestSubagentLifecycleEventTypes(t *testing.T) {
	// Verify all event types can be created and published without panic.
	bus := NewSubagentBus()
	for _, evtType := range []SubagentEventType{
		SubagentEventUnspecified,
		SubagentEventSpawned,
		SubagentEventHeartbeat,
		SubagentEventCompleted,
		SubagentEventFailed,
		SubagentEventKilled,
	} {
		bus.Publish(SubagentLifecycleEvent{EventType: evtType, TaskID: "test"})
	}
}

