package interop

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/proto/interop"
	"google.golang.org/protobuf/proto"
)

// ReliableMesh wraps TeammateMesh to provide guaranteed delivery semantics
// via explicit JobAck messages and retry logic.
type ReliableMesh interface {
	PublishReliable(ctx context.Context, channel string, dispatch *interoppb.JobDispatch) error
	SubscribeReliable(ctx context.Context, channel string, handler func(*interoppb.JobDispatch) error) error
}

type reliableMeshImpl struct {
	baseMesh TeammateMesh
	ackChan  string

	// Pending acks tracking
	mu      sync.RWMutex
	pending map[string]chan bool
}

// NewReliableMesh creates a new ReliableMesh on top of an existing TeammateMesh.
func NewReliableMesh(baseMesh TeammateMesh, ackChannel string) (ReliableMesh, error) {
	rm := &reliableMeshImpl{
		baseMesh: baseMesh,
		ackChan:  ackChannel,
		pending:  make(map[string]chan bool),
	}

	// Start background subscriber for acks
	go rm.listenForAcks()

	return rm, nil
}

func (rm *reliableMeshImpl) listenForAcks() {
	ctx := context.Background()
	sub, err := rm.baseMesh.Subscribe(ctx, rm.ackChan)
	if err != nil {
		return // In a real system, we'd log this or retry
	}

	for msg := range sub {
		var ack interoppb.JobAck
		if err := proto.Unmarshal(msg, &ack); err != nil {
			continue
		}

		rm.mu.RLock()
		ch, ok := rm.pending[ack.JobId]
		rm.mu.RUnlock()

		if ok {
			select {
			case ch <- ack.Success:
			default:
			}
		}
	}
}

func (rm *reliableMeshImpl) PublishReliable(ctx context.Context, channel string, dispatch *interoppb.JobDispatch) error {
	data, err := proto.Marshal(dispatch)
	if err != nil {
		return fmt.Errorf("failed to marshal dispatch: %w", err)
	}

	ackCh := make(chan bool, 1)
	rm.mu.Lock()
	rm.pending[dispatch.JobId] = ackCh
	rm.mu.Unlock()

	defer func() {
		rm.mu.Lock()
		delete(rm.pending, dispatch.JobId)
		rm.mu.Unlock()
	}()

	maxRetries := int(dispatch.MaxRetries)
	if maxRetries <= 0 {
		maxRetries = 3
	}

	backoff := 100 * time.Millisecond

	for attempt := 0; attempt < maxRetries; attempt++ {
		if err := rm.baseMesh.Publish(ctx, channel, data); err != nil {
			// Publish error, wait and retry
			time.Sleep(backoff)
			backoff *= 2
			continue
		}

		// Wait for ack or timeout
		select {
		case success := <-ackCh:
			if success {
				return nil // Delivered successfully
			}
			// NACK received, retry
		case <-time.After(2 * time.Second):
			// Timeout, retry
		case <-ctx.Done():
			return ctx.Err()
		}

		backoff *= 2
	}

	return fmt.Errorf("failed to deliver job %s after %d attempts", dispatch.JobId, maxRetries)
}

func (rm *reliableMeshImpl) SubscribeReliable(ctx context.Context, channel string, handler func(*interoppb.JobDispatch) error) error {
	sub, err := rm.baseMesh.Subscribe(ctx, channel)
	if err != nil {
		return fmt.Errorf("failed to subscribe: %w", err)
	}

	go func() {
		for {
			select {
			case msg, ok := <-sub:
				if !ok {
					return
				}

				var dispatch interoppb.JobDispatch
				if err := proto.Unmarshal(msg, &dispatch); err != nil {
					continue
				}

				// Process the job
				success := handler(&dispatch) == nil

				// Send Ack
				ack := &interoppb.JobAck{
					JobId:     dispatch.JobId,
					AgentId:   "system", // Ideally we'd have the agent's real ID here
					Timestamp: time.Now().Unix(),
					Success:   success,
				}

				if ackData, err := proto.Marshal(ack); err == nil {
					_ = rm.baseMesh.Publish(ctx, rm.ackChan, ackData)
				}

			case <-ctx.Done():
				return
			}
		}
	}()

	return nil
}
