package interop

import (
	"context"
	"fmt"
	"time"

	"onehumancorp/srcs/server/orchestration"
	pb "onehumancorp/srcs/server/pb/interop"
	"github.com/golang/protobuf/proto"
)

type InteropProtocol struct {
	bus    orchestration.MeshTransport
	lock   DistributedLock
	nodeID string
}

func NewInteropProtocol(bus orchestration.MeshTransport, lock DistributedLock, nodeID string) *InteropProtocol {
	return &InteropProtocol{
		bus:    bus,
		lock:   lock,
		nodeID: nodeID,
	}
}

func (p *InteropProtocol) Handoff(ctx context.Context, missionID, tenantID string, statePayload []byte) error {
	lockResource := fmt.Sprintf("handoff:%s", missionID)

	// Wait for lock with backoff
	retries := 0
	for {
		acquired, err := p.lock.AcquireLock(ctx, lockResource, p.nodeID, 10)
		if err != nil {
			return err
		}
		if acquired {
			break
		}

		retries++
		if retries >= 10 { // Approx 5s timeout if sleep is ~50ms average, let's keep it simple
			return fmt.Errorf("Timeout waiting for lock")
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		case <-time.After(time.Duration(50*retries) * time.Millisecond):
		}
	}
	defer p.lock.ReleaseLock(context.WithoutCancel(ctx), lockResource, p.nodeID)

	idempotencyResource := fmt.Sprintf("handoff:processed:%s", missionID)
	attemptOwner := fmt.Sprintf("%s_%d", p.nodeID, time.Now().UnixNano())

	// Hold the idempotency lock only briefly while publishing
	// If it fails to acquire, it means another node successfully published recently.
	acquired, err := p.lock.AcquireLock(ctx, idempotencyResource, attemptOwner, 30)
	if err != nil || !acquired {
		return nil
	}

	msg := &pb.StateHandoff{
		MissionId:         missionID,
		TenantId:          tenantID,
		SourceMode:        pb.DeploymentMode_MODE_UNSPECIFIED,
		TargetMode:        pb.DeploymentMode_MODE_UNSPECIFIED,
		TimestampMs:       time.Now().UnixMilli(),
		StateSnapshotJson: statePayload,
	}

	buf, err := proto.Marshal(msg)
	if err != nil {
		return err
	}

	retries = 0
	delayMs := 100
	for {
		err = p.bus.Publish(ctx, "system:state_handoff", buf)
		if err == nil {
			// Successfully published, now persist idempotency lock for 1 hour to prevent duplicates
			p.lock.AcquireLock(context.WithoutCancel(ctx), idempotencyResource, attemptOwner, 3600)
			break
		}
		if retries >= 5 {
			return fmt.Errorf("Failed to publish state handoff after retries: %v", err)
		}
		retries++

		select {
		case <-ctx.Done():
			return ctx.Err()
		case <-time.After(time.Duration(delayMs) * time.Millisecond):
		}

		delayMs *= 2
	}

	return nil
}

func (p *InteropProtocol) ListenForStateHandoff(ctx context.Context, handler func(*pb.StateHandoff)) error {
	return p.bus.Subscribe(ctx, "system:state_handoff", func(data []byte) {
		var msg pb.StateHandoff
		if err := proto.Unmarshal(data, &msg); err == nil {
			handler(&msg)
		}
	})
}

func (p *InteropProtocol) ListenForPings(ctx context.Context) error {
	return p.bus.Subscribe(ctx, "system:health_ping", func(data []byte) {
		var msg pb.HealthPing
		if err := proto.Unmarshal(data, &msg); err == nil {
			ack := &pb.HealthAck{
				SourceNodeId: p.nodeID,
				TargetNodeId: msg.SourceNodeId,
				TimestampMs:  time.Now().UnixMilli(),
			}
			if buf, err := proto.Marshal(ack); err == nil {
				ackTopic := fmt.Sprintf("system:health_ack:%s", msg.SourceNodeId)
				go func() {
					retries := 0
					delayMs := 50

					ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
					defer cancel()

					for retries < 5 {
						if err := p.bus.Publish(ctx, ackTopic, buf); err == nil {
							break
						}
						retries++

						select {
						case <-ctx.Done():
							return
						case <-time.After(time.Duration(delayMs) * time.Millisecond):
						}

						delayMs *= 2
					}
				}()
			}
		}
	})
}

func (p *InteropProtocol) CheckHealth(ctx context.Context, timeoutMs uint64) (bool, error) {
	ackTopic := fmt.Sprintf("system:health_ack:%s", p.nodeID)
	received := make(chan struct{}, 1)

	subCtx, cancelSub := context.WithCancel(ctx)
	defer cancelSub()

	err := p.bus.Subscribe(subCtx, ackTopic, func(data []byte) {
		select {
		case received <- struct{}{}:
		default:
		}
	})
	if err != nil {
		return false, err
	}

	ping := &pb.HealthPing{
		SourceNodeId: p.nodeID,
		CurrentMode:  pb.DeploymentMode_MODE_UNSPECIFIED,
		TimestampMs:  time.Now().UnixMilli(),
	}
	buf, err := proto.Marshal(ping)
	if err != nil {
		return false, err
	}

	if err := p.bus.Publish(ctx, "system:health_ping", buf); err != nil {
		return false, err
	}

	select {
	case <-received:
		return true, nil
	case <-time.After(time.Duration(timeoutMs) * time.Millisecond):
		return false, nil
	case <-ctx.Done():
		return false, ctx.Err()
	}
}

func (p *InteropProtocol) DispatchJob(ctx context.Context, jobID, tenantID, actionName string, payload []byte, timeoutMs uint64) (bool, error) {
	ackTopic := fmt.Sprintf("system:job_ack:%s", jobID)
	received := make(chan struct{}, 1)

	subCtx, cancelSub := context.WithCancel(ctx)
	defer cancelSub()

	err := p.bus.Subscribe(subCtx, ackTopic, func(data []byte) {
		select {
		case received <- struct{}{}:
		default:
		}
	})
	if err != nil {
		return false, err
	}

	dispatch := &pb.JobDispatch{
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

	dispatchTopic := fmt.Sprintf("system:job_dispatch:%s", tenantID)

	retries := 0
	delayMs := 100
	for {
		err = p.bus.Publish(ctx, dispatchTopic, buf)
		if err == nil {
			break
		}
		if retries >= 5 {
			return false, fmt.Errorf("Failed to publish job dispatch after retries: %v", err)
		}
		retries++

		select {
		case <-ctx.Done():
			return false, ctx.Err()
		case <-time.After(time.Duration(delayMs) * time.Millisecond):
		}

		delayMs *= 2
	}

	select {
	case <-received:
		return true, nil
	case <-time.After(time.Duration(timeoutMs) * time.Millisecond):
		return false, nil
	case <-ctx.Done():
		return false, ctx.Err()
	}
}

func (p *InteropProtocol) ListenForJobs(ctx context.Context, tenantID string, handler func(*pb.JobDispatch)) error {
	return p.bus.Subscribe(ctx, fmt.Sprintf("system:job_dispatch:%s", tenantID), func(data []byte) {
		var msg pb.JobDispatch
		if err := proto.Unmarshal(data, &msg); err == nil {
			handler(&msg)
		}
	})
}

func (p *InteropProtocol) ReportJobStatus(ctx context.Context, jobID, tenantID, status string, details []byte) error {
	update := &pb.JobStatusUpdate{
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

	retries := 0
	delayMs := 100
	topic := fmt.Sprintf("system:job_status:%s", jobID)
	for {
		err = p.bus.Publish(ctx, topic, buf)
		if err == nil {
			return nil
		}
		if retries >= 5 {
			return fmt.Errorf("Failed to publish job status update after retries: %v", err)
		}
		retries++

		select {
		case <-ctx.Done():
			return ctx.Err()
		case <-time.After(time.Duration(delayMs) * time.Millisecond):
		}

		delayMs *= 2
	}
}

func (p *InteropProtocol) ListenForJobStatus(ctx context.Context, jobID string, handler func(*pb.JobStatusUpdate)) error {
	return p.bus.Subscribe(ctx, fmt.Sprintf("system:job_status:%s", jobID), func(data []byte) {
		var msg pb.JobStatusUpdate
		if err := proto.Unmarshal(data, &msg); err == nil {
			handler(&msg)
		}
	})
}

func (p *InteropProtocol) AckJob(ctx context.Context, jobID string) error {
	ack := &pb.JobAck{
		JobId:       jobID,
		NodeId:      p.nodeID,
		TimestampMs: time.Now().UnixMilli(),
	}
	buf, err := proto.Marshal(ack)
	if err != nil {
		return err
	}

	ackTopic := fmt.Sprintf("system:job_ack:%s", jobID)
	return p.bus.Publish(ctx, ackTopic, buf)
}
