package interop

import (
	"context"
	"fmt"
	"sync/atomic"
	"time"

	pb "onehumancorp/srcs/server/interop/pb"
	"github.com/golang/protobuf/proto"
)

type Transport interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string, handler func(data []byte)) error
	AcquireLock(ctx context.Context, resource, owner string, ttlSeconds int) (bool, error)
	ReleaseLock(ctx context.Context, resource, owner string) error
}

type InteropProtocol struct {
	transport Transport
	nodeID    string
}

func NewInteropProtocol(transport Transport, nodeID string) *InteropProtocol {
	return &InteropProtocol{
		transport: transport,
		nodeID:    nodeID,
	}
}

func (p *InteropProtocol) Handoff(ctx context.Context, missionID, tenantID string, statePayload []byte) error {
	lockResource := fmt.Sprintf("handoff:%s", missionID)

	ctxTimeout, cancel := context.WithTimeout(ctx, 5*time.Second)
	defer cancel()

	var acquired bool
	var retries int
	for {
		ok, err := p.transport.AcquireLock(ctxTimeout, lockResource, p.nodeID, 10)
		if err == nil && ok {
			acquired = true
			break
		}
		if ctxTimeout.Err() != nil {
			return fmt.Errorf("timeout waiting for handoff lock")
		}
		retries++
		time.Sleep(time.Duration(50*retries) * time.Millisecond)
	}

	if !acquired {
		return fmt.Errorf("could not acquire lock")
	}

	idempotencyResource := fmt.Sprintf("handoff:processed:%s", missionID)
	attemptOwner := fmt.Sprintf("%s_%d", p.nodeID, time.Now().UnixNano())

	if ok, _ := p.transport.AcquireLock(ctx, idempotencyResource, attemptOwner, 3600); !ok {
		_ = p.transport.ReleaseLock(ctx, lockResource, p.nodeID)
		return nil // already processed
	}

	msg := &pb.StateHandoff{
		SourceMode:        0,
		TargetMode:        0,
		MissionId:         missionID,
		TenantId:          tenantID,
		TimestampMs:       time.Now().UnixMilli(),
		StateSnapshotJson: statePayload,
	}

	buf, err := proto.Marshal(msg)
	if err != nil {
		_ = p.transport.ReleaseLock(ctx, idempotencyResource, attemptOwner)
		_ = p.transport.ReleaseLock(ctx, lockResource, p.nodeID)
		return err
	}

	err = p.transport.Publish(ctx, "system:state_handoff", buf)
	if err != nil {
		_ = p.transport.ReleaseLock(ctx, idempotencyResource, attemptOwner)
	}
	_ = p.transport.ReleaseLock(ctx, lockResource, p.nodeID)

	return err
}

func (p *InteropProtocol) CheckHealth(ctx context.Context, timeoutMs int) (bool, error) {
	ackTopic := fmt.Sprintf("system:health_ack:%s", p.nodeID)

	var received int32
	ctxSub, cancelSub := context.WithCancel(ctx)
	defer cancelSub()

	err := p.transport.Subscribe(ctxSub, ackTopic, func(data []byte) {
		atomic.StoreInt32(&received, 1)
	})
	if err != nil {
		return false, err
	}

	ping := &pb.HealthPing{
		CurrentMode:    0,
		TimestampMs:    time.Now().UnixMilli(),
		SourceNodeId:   p.nodeID,
	}
	buf, _ := proto.Marshal(ping)

	if err := p.transport.Publish(ctx, "system:health_ping", buf); err != nil {
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

func (p *InteropProtocol) DispatchJob(ctx context.Context, jobID, tenantID, actionName string, payload []byte, timeoutMs int) (bool, error) {
	ackTopic := fmt.Sprintf("system:job_ack:%s", jobID)

	var received int32
	ctxSub, cancelSub := context.WithCancel(ctx)
	defer cancelSub()

	err := p.transport.Subscribe(ctxSub, ackTopic, func(data []byte) {
		atomic.StoreInt32(&received, 1)
	})
	if err != nil {
		return false, err
	}

	dispatch := &pb.JobDispatch{
		JobId:        jobID,
		TenantId:     tenantID,
		ActionName:   actionName,
		PayloadJson:  payload,
		TimestampMs:  time.Now().UnixMilli(),
	}
	buf, _ := proto.Marshal(dispatch)

	if err := p.transport.Publish(ctx, fmt.Sprintf("system:job_dispatch:%s", tenantID), buf); err != nil {
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
