package orchestration

import (
	"context"
	"sync"
	"sync/atomic"
	"testing"
)

func TestLocalTeammateMesh_PublishSubscribe(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx := context.Background()
	channel := "test_channel"

	var messageCount int32
	var wg sync.WaitGroup

	numMessages := 100
	wg.Add(numMessages)

	// Subscribe to the channel
	err := mesh.Subscribe(ctx, channel, func(data []byte) {
		atomic.AddInt32(&messageCount, 1)
		wg.Done()
	})

	if err != nil {
		t.Fatalf("expected no error on Subscribe, got %v", err)
	}

	// Publish messages concurrently
	var pubWg sync.WaitGroup
	for i := 0; i < numMessages; i++ {
		pubWg.Add(1)
		go func(msgIdx int) {
			defer pubWg.Done()
			err := mesh.Publish(ctx, channel, []byte("test_message"))
			if err != nil {
				t.Errorf("expected no error on Publish, got %v", err)
			}
		}(i)
	}

	pubWg.Wait()
	wg.Wait()

	if atomic.LoadInt32(&messageCount) != int32(numMessages) {
		t.Errorf("expected %d messages, got %d", numMessages, messageCount)
	}
}
