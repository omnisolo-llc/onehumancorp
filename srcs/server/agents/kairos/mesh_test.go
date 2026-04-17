package kairos

import (
	"context"
	"encoding/json"
	"fmt"
	"testing"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
)

type mockPubSub struct {
	ch      chan *redis.Message
	err     error
	closed  bool
}

func (m *mockPubSub) Receive(ctx context.Context) (interface{}, error) {
	if m.err != nil {
		return nil, m.err
	}
	return nil, nil
}

func (m *mockPubSub) Channel() <-chan *redis.Message {
	return m.ch
}

func (m *mockPubSub) Close() error {
	m.closed = true
	return nil
}

type mockRedisClient struct {
	pubsubs map[string]*mockPubSub
	err     error
}

func newMockRedisClient() *mockRedisClient {
	return &mockRedisClient{
		pubsubs: make(map[string]*mockPubSub),
	}
}

// Just returns a dummy ok response since we aren't testing intcmd
func (m *mockRedisClient) Publish(ctx context.Context, channel string, message interface{}) *redis.IntCmd {
    if m.err != nil {
		return redis.NewIntResult(0, m.err)
	}
	if ps, ok := m.pubsubs[channel]; ok && ps.ch != nil {
        var payloadStr string
        switch v := message.(type) {
        case []byte:
            payloadStr = string(v)
        case string:
            payloadStr = v
        default:
            b, _ := json.Marshal(v)
            payloadStr = string(b)
        }

		ps.ch <- &redis.Message{
			Channel: channel,
			Payload: payloadStr,
		}
	}
	return redis.NewIntResult(1, nil)
}

func (m *mockRedisClient) Subscribe(ctx context.Context, channels ...string) PubSubSubscription {
	channel := channels[0]
	ps := &mockPubSub{
		ch:  make(chan *redis.Message, 10),
		err: m.err,
	}
	m.pubsubs[channel] = ps
	return ps
}


func TestTeammateMeshHermetic(t *testing.T) {
	mockClient := newMockRedisClient()
	mesh := NewTeammateMeshWithInterface(mockClient)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	t.Run("TaskCreated", func(t *testing.T) {
		ch, err := mesh.SubscribeTaskCreated(ctx)
		if err != nil {
			t.Fatalf("failed to subscribe: %v", err)
		}

		mission := &Mission{
			ID:     uuid.New(),
			Title:  "Test Mission",
			Status: "PENDING",
		}
		err = mesh.PublishTaskCreated(ctx, mission)
		if err != nil {
			t.Fatalf("failed to publish: %v", err)
		}

		select {
		case <-time.After(1 * time.Second):
			t.Fatalf("timeout waiting for task")
		case received := <-ch:
			if received.ID != mission.ID {
				t.Errorf("expected mission ID %s, got %s", mission.ID, received.ID)
			}
		}

        // Test bad json on channel
        mockClient.pubsubs[TaskCreatedChannel].ch <- &redis.Message{Payload: "bad json"}

        // Let it process
        time.Sleep(10 * time.Millisecond)
	})

	t.Run("StatusUpdate", func(t *testing.T) {
		ch, err := mesh.SubscribeStatusUpdate(ctx)
		if err != nil {
			t.Fatalf("failed to subscribe: %v", err)
		}

		missionID := uuid.New()
		err = mesh.PublishStatusUpdate(ctx, missionID, "IN_PROGRESS")
		if err != nil {
			t.Fatalf("failed to publish: %v", err)
		}

		select {
		case <-time.After(1 * time.Second):
			t.Fatalf("timeout waiting for status")
		case received := <-ch:
			if received.MissionID != missionID {
				t.Errorf("expected mission ID %s, got %s", missionID, received.MissionID)
			}
			if received.Status != "IN_PROGRESS" {
				t.Errorf("expected status IN_PROGRESS, got %s", received.Status)
			}
		}

        // Test bad json
        mockClient.pubsubs[StatusUpdateChannel].ch <- &redis.Message{Payload: "bad json"}
	})

	t.Run("DirectMessage", func(t *testing.T) {
		agentID := "agent-123"
		ch, err := mesh.SubscribeDirectMessages(ctx, agentID)
		if err != nil {
			t.Fatalf("failed to subscribe: %v", err)
		}

		msg := []byte("hello agent")
		err = mesh.PublishDirectMessage(ctx, agentID, msg)
		if err != nil {
			t.Fatalf("failed to publish: %v", err)
		}

		select {
		case <-time.After(1 * time.Second):
			t.Fatalf("timeout waiting for message")
		case received := <-ch:
			if string(received) != "hello agent" {
				t.Errorf("expected 'hello agent', got '%s'", string(received))
			}
		}
	})

	t.Run("Errors", func(t *testing.T) {
		errClient := newMockRedisClient()
		errClient.err = fmt.Errorf("redis connection error")
		errMesh := NewTeammateMeshWithInterface(errClient)

		_, err := errMesh.SubscribeTaskCreated(ctx)
		if err == nil {
			t.Errorf("expected error on subscribe task created")
		}

		_, err = errMesh.SubscribeStatusUpdate(ctx)
		if err == nil {
			t.Errorf("expected error on subscribe status update")
		}

		_, err = errMesh.SubscribeDirectMessages(ctx, "agent-fail")
		if err == nil {
			t.Errorf("expected error on subscribe direct message")
		}

		mission := &Mission{ID: uuid.New()}
		err = errMesh.PublishTaskCreated(ctx, mission)
		if err == nil {
			t.Errorf("expected error on publish task created")
		}

		err = errMesh.PublishStatusUpdate(ctx, mission.ID, "DONE")
		if err == nil {
			t.Errorf("expected error on publish status update")
		}

		err = errMesh.PublishDirectMessage(ctx, "agent-fail", []byte("fail"))
		if err == nil {
			t.Errorf("expected error on publish direct message")
		}
	})

    t.Run("ContextCancellation", func(t *testing.T) {
        cancelCtx, cancelFunc := context.WithCancel(context.Background())
        ch, _ := mesh.SubscribeTaskCreated(cancelCtx)
        cancelFunc()

        // wait for channel to close
        select {
        case <-time.After(1 * time.Second):
            t.Fatalf("timeout waiting for channel to close after cancellation")
        case _, ok := <-ch:
            if ok {
                t.Fatalf("expected channel to be closed")
            }
        }

        // confirm pubsub was closed
        if !mockClient.pubsubs[TaskCreatedChannel].closed {
            t.Errorf("expected pubsub to be closed")
        }
    })

    t.Run("ContextCancellationStatus", func(t *testing.T) {
        cancelCtx, cancelFunc := context.WithCancel(context.Background())
        ch, _ := mesh.SubscribeStatusUpdate(cancelCtx)
        cancelFunc()

        select {
        case <-time.After(1 * time.Second):
            t.Fatalf("timeout waiting for channel to close after cancellation")
        case _, ok := <-ch:
            if ok {
                t.Fatalf("expected channel to be closed")
            }
        }
    })

    t.Run("ContextCancellationDirectMessage", func(t *testing.T) {
        cancelCtx, cancelFunc := context.WithCancel(context.Background())
        ch, _ := mesh.SubscribeDirectMessages(cancelCtx, "agent-1")
        cancelFunc()

        select {
        case <-time.After(1 * time.Second):
            t.Fatalf("timeout waiting for channel to close after cancellation")
        case _, ok := <-ch:
            if ok {
                t.Fatalf("expected channel to be closed")
            }
        }
    })

    t.Run("RealClientInitialization", func(t *testing.T) {
        // Just to cover the NewTeammateMesh block
        rdb := redis.NewClient(&redis.Options{})
        defer rdb.Close()
        _ = NewTeammateMesh(rdb)
    })

    t.Run("StatusUpdateEvent_JSON", func(t *testing.T) {
        event := StatusUpdateEvent{
            MissionID: uuid.New(),
            Status:    "COMPLETED",
            Timestamp: time.Now().UTC(),
        }

        data, err := json.Marshal(event)
        if err != nil {
            t.Errorf("failed to marshal event: %v", err)
        }

        var decoded StatusUpdateEvent
        err = json.Unmarshal(data, &decoded)
        if err != nil {
            t.Errorf("failed to unmarshal event: %v", err)
        }

        if decoded.Status != "COMPLETED" {
            t.Errorf("expected COMPLETED, got %s", decoded.Status)
        }
    })
}
