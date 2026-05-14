package interop

import (
	"context"
	"fmt"
	"sync/atomic"
	"time"

	"github.com/golang/protobuf/proto"

	interoppb "onehumancorp/srcs/server/pb/interop"
)

type InteropProtocol struct {
	bus    Bus
	lock   DistributedLock
	nodeID string
}

func NewInteropProtocol(bus Bus, lock DistributedLock, nodeID string) *InteropProtocol {
	return &InteropProtocol{
		bus:    bus,
		lock:   lock,
		nodeID: nodeID,
	}
}

func (p *InteropProtocol) Handoff(ctx context.Context, missionID, tenantID string, statePayload []byte) error {
	lockResource := fmt.Sprintf("handoff:%s", missionID)

	var lockAcquired bool
	for retries := 0; retries < 100; retries++ {
		ok, err := p.lock.AcquireLock(ctx, lockResource, p.nodeID, 10)
		if err == nil && ok {
			lockAcquired = true
			break
		}
		select {
		case <-ctx.Done():
			return ctx.Err()
		case <-time.After(time.Duration(50*(retries+1)) * time.Millisecond):
		}
	}

	if !lockAcquired {
		return fmt.Errorf("timeout waiting for lock")
	}
	defer p.lock.ReleaseLock(ctx, lockResource, p.nodeID)

	idempotencyLockResource := fmt.Sprintf("handoff:processed:%s", missionID)
	attemptOwner := fmt.Sprintf("%s_%d", p.nodeID, time.Now().UnixNano())

	ok, err := p.lock.AcquireLock(ctx, idempotencyLockResource, attemptOwner, 3600)
	if err != nil || !ok {
		return nil
	}

	msgProto := &interoppb.StateHandoff{
		SourceMode:        0,
		TargetMode:        0,
		MissionId:         missionID,
		TenantId:          tenantID,
		TimestampMs:       time.Now().UnixMilli(),
		StateSnapshotJson: statePayload,
	}

	buf, err := proto.Marshal(msgProto)
	if err != nil {
		p.lock.ReleaseLock(ctx, idempotencyLockResource, attemptOwner)
		return fmt.Errorf("failed to marshal handoff message: %w", err)
	}

	msg := Message{
		Topic:   "system:state_handoff",
		Payload: buf,
	}

	delayMs := 100
	for retries := 0; retries < 5; retries++ {
		err := p.bus.Publish(ctx, msg)
		if err == nil {
			return nil
		}
		select {
		case <-ctx.Done():
			p.lock.ReleaseLock(context.Background(), idempotencyLockResource, attemptOwner)
			return ctx.Err()
		case <-time.After(time.Duration(delayMs) * time.Millisecond):
			delayMs *= 2
		}
	}

	p.lock.ReleaseLock(context.Background(), idempotencyLockResource, attemptOwner)
	return fmt.Errorf("failed to publish state handoff after retries")
}

func (p *InteropProtocol) ListenForStateHandoff(ctx context.Context, handler func(*interoppb.StateHandoff)) (func(), error) {
	busHandler := func(msg Message) {
		if msg.Topic == "system:state_handoff" {
			var handoff interoppb.StateHandoff
			if err := proto.Unmarshal(msg.Payload, &handoff); err == nil {
				handler(&handoff)
			}
		}
	}
	return p.bus.Subscribe(ctx, "system:state_handoff", busHandler)
}

func (p *InteropProtocol) ListenForPings(ctx context.Context) (func(), error) {
	nodeID := p.nodeID
	bus := p.bus

	handler := func(msg Message) {
		if msg.Topic == "system:health_ping" {
			var ping interoppb.HealthPing
			if err := proto.Unmarshal(msg.Payload, &ping); err == nil {
				ack := &interoppb.HealthAck{
					SourceNodeId: nodeID,
					TargetNodeId: ping.SourceNodeId,
					TimestampMs:  time.Now().UnixMilli(),
				}

				buf, err := proto.Marshal(ack)
				if err == nil {
					ackMsg := Message{
						Topic:   fmt.Sprintf("system:health_ack:%s", ping.SourceNodeId),
						Payload: buf,
					}

					go func() {
						delayMs := 50
						for retries := 0; retries < 5; retries++ {
							if err := bus.Publish(context.Background(), ackMsg); err == nil {
								break
							}
							time.Sleep(time.Duration(delayMs) * time.Millisecond)
							delayMs *= 2
						}
					}()
				}
			}
		}
	}

	return p.bus.Subscribe(ctx, "system:health_ping", handler)
}

func (p *InteropProtocol) CheckHealth(ctx context.Context, timeoutMs uint64) (bool, error) {
	var received int32

	ackTopic := fmt.Sprintf("system:health_ack:%s", p.nodeID)
	handler := func(msg Message) {
		if msg.Topic == ackTopic {
			atomic.StoreInt32(&received, 1)
		}
	}

	cancel, err := p.bus.Subscribe(ctx, ackTopic, handler)
	if err != nil {
		return false, err
	}
	defer cancel()

	ping := &interoppb.HealthPing{
		SourceNodeId: p.nodeID,
		TimestampMs:  time.Now().UnixMilli(),
		CurrentMode:  0,
	}

	buf, err := proto.Marshal(ping)
	if err != nil {
		return false, err
	}

	msg := Message{
		Topic:   "system:health_ping",
		Payload: buf,
	}

	if err := p.bus.Publish(ctx, msg); err != nil {
		return false, err
	}

	start := time.Now()
	for time.Since(start).Milliseconds() < int64(timeoutMs) {
		if atomic.LoadInt32(&received) == 1 {
			return true, nil
		}
		time.Sleep(10 * time.Millisecond)
	}

	return false, nil
}

func (p *InteropProtocol) DispatchJob(ctx context.Context, jobID, tenantID, actionName string, payload []byte, timeoutMs uint64) (bool, error) {
	var received int32

	ackTopic := fmt.Sprintf("system:job_ack:%s", jobID)
	handler := func(msg Message) {
		if msg.Topic == ackTopic {
			atomic.StoreInt32(&received, 1)
		}
	}

	cancel, err := p.bus.Subscribe(ctx, ackTopic, handler)
	if err != nil {
		return false, err
	}
	defer cancel()

	dispatch := &interoppb.JobDispatch{
		JobId:       jobID,
		TenantId:    tenantID,
		ActionName:  actionName,
		PayloadJson: payload,
		TimestampMs: time.Now().UnixMilli(),
	}

	buf, err := proto.Marshal(dispatch)
	if err != nil {
		return false, err
	}

	msg := Message{
		Topic:   fmt.Sprintf("system:job_dispatch:%s", tenantID),
		Payload: buf,
	}

	delayMs := 100
	var publishErr error
	for retries := 0; retries < 5; retries++ {
		publishErr = p.bus.Publish(ctx, msg)
		if publishErr == nil {
			break
		}
		select {
		case <-ctx.Done():
			return false, ctx.Err()
		case <-time.After(time.Duration(delayMs) * time.Millisecond):
			delayMs *= 2
		}
	}
	if publishErr != nil {
		return false, fmt.Errorf("failed to publish job dispatch after retries: %w", publishErr)
	}

	start := time.Now()
	for time.Since(start).Milliseconds() < int64(timeoutMs) {
		if atomic.LoadInt32(&received) == 1 {
			return true, nil
		}
		time.Sleep(10 * time.Millisecond)
	}

	return false, nil
}

func (p *InteropProtocol) ListenForJobs(ctx context.Context, tenantID string, processor func(*interoppb.JobDispatch)) (func(), error) {
	nodeID := p.nodeID
	bus := p.bus

	handler := func(msg Message) {
		var decoded interoppb.JobDispatch
		if err := proto.Unmarshal(msg.Payload, &decoded); err == nil {
			if processor != nil {
				processor(&decoded)
			}
			ack := &interoppb.JobAck{
				JobId:       decoded.JobId,
				NodeId:      nodeID,
				TimestampMs: time.Now().UnixMilli(),
			}

			if buf, err := proto.Marshal(ack); err == nil {
				ackMsg := Message{
					Topic:   fmt.Sprintf("system:job_ack:%s", decoded.JobId),
					Payload: buf,
				}
				go func() {
					delayMs := 50
					for retries := 0; retries < 5; retries++ {
						if err := bus.Publish(context.Background(), ackMsg); err == nil {
							break
						}
						time.Sleep(time.Duration(delayMs) * time.Millisecond)
						delayMs *= 2
					}
				}()
			}
		}
	}

	return p.bus.Subscribe(ctx, fmt.Sprintf("system:job_dispatch:%s", tenantID), handler)
}

func (p *InteropProtocol) ReportJobStatus(ctx context.Context, jobID, tenantID, status string, details []byte) error {
	update := &interoppb.JobStatusUpdate{
		JobId:          jobID,
		TenantId:       tenantID,
		Status:         status,
		DetailsPayload: details,
		TimestampMs:    time.Now().UnixMilli(),
	}

	buf, err := proto.Marshal(update)
	if err != nil {
		return err
	}

	msg := Message{
		Topic:   fmt.Sprintf("system:job_status:%s", jobID),
		Payload: buf,
	}

	delayMs := 100
	for retries := 0; retries < 5; retries++ {
		if err := p.bus.Publish(ctx, msg); err == nil {
			return nil
		}
		select {
		case <-ctx.Done():
			return ctx.Err()
		case <-time.After(time.Duration(delayMs) * time.Millisecond):
			delayMs *= 2
		}
	}

	return fmt.Errorf("failed to publish job status update after retries")
}

func (p *InteropProtocol) ListenForJobStatus(ctx context.Context, jobID string, handler func(*interoppb.JobStatusUpdate)) (func(), error) {
	busHandler := func(msg Message) {
		var decoded interoppb.JobStatusUpdate
		if err := proto.Unmarshal(msg.Payload, &decoded); err == nil {
			handler(&decoded)
		}
	}

	return p.bus.Subscribe(ctx, fmt.Sprintf("system:job_status:%s", jobID), busHandler)
}
