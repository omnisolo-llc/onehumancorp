package interop

import (
	"context"
	"fmt"
	"time"

	"google.golang.org/protobuf/proto"
	"onehumancorp/srcs/server/orchestration/kairos"
	"onehumancorp/srcs/server/pb/interoppb"
)

type Protocol struct {
	mesh   kairos.TeammateMesh
	nodeID string
}

func NewProtocol(mesh kairos.TeammateMesh, nodeID string) *Protocol {
	return &Protocol{
		mesh:   mesh,
		nodeID: nodeID,
	}
}

// Handoff triggers a state handoff when switching modes using protobuf on the wire
func (p *Protocol) Handoff(ctx context.Context, missionID string, tenantID string, statePayload []byte) error {
	lockResource := fmt.Sprintf("handoff:%s", missionID)

	// Wait for lock with a timeout to prevent deadlocks and apply backoff.
	acquireCtx, cancel := context.WithTimeout(ctx, 5*time.Second)
	defer cancel()

	acquired := false
	retries := 0
	for {
		ok, err := p.mesh.AcquireLock(acquireCtx, lockResource, 10*time.Second)
		if err == nil && ok {
			acquired = true
			break
		}

		select {
		case <-acquireCtx.Done():
			return fmt.Errorf("timeout waiting for lock")
		case <-time.After(time.Duration(50*retries) * time.Millisecond):
			retries++
		}
	}

	if !acquired {
		return fmt.Errorf("timeout waiting for lock")
	}

	defer p.mesh.ReleaseLock(context.Background(), lockResource)

	// Idempotency check
	idempotencyLockResource := fmt.Sprintf("handoff:processed:%s", missionID)
	var _ = fmt.Sprintf("%s_%d", p.nodeID, time.Now().UnixNano())

	ok, err := p.mesh.AcquireLock(ctx, idempotencyLockResource, 3600*time.Second)
	if err != nil {
		return err
	}
	if !ok {
		// Already processed
		return nil
	}

	handoffMsg := &interoppb.StateHandoff{
		SourceMode:        interoppb.DeploymentMode_MODE_UNSPECIFIED,
		TargetMode:        interoppb.DeploymentMode_MODE_UNSPECIFIED,
		MissionId:         missionID,
		TenantId:          tenantID,
		TimestampMs:       time.Now().UnixMilli(),
		StateSnapshotJson: statePayload,
	}

	buf, err := proto.Marshal(handoffMsg)
	if err != nil {
		p.mesh.ReleaseLock(context.Background(), idempotencyLockResource)
		return err
	}

	retries = 0
	delayMs := 100
	for {
		err := p.mesh.Publish(ctx, "system:state_handoff", buf)
		if err == nil {
			return nil
		}

		if retries >= 5 {
			p.mesh.ReleaseLock(context.Background(), idempotencyLockResource)
			return fmt.Errorf("failed to publish state handoff after retries: %w", err)
		}

		time.Sleep(time.Duration(delayMs) * time.Millisecond)
		delayMs *= 2
		retries++
	}
}

// ListenForStateHandoff listens for state handoff updates
func (p *Protocol) ListenForStateHandoff(ctx context.Context, handler func(*interoppb.StateHandoff)) (kairos.Subscription, error) {
	return p.mesh.Subscribe(ctx, "system:state_handoff", func(msg []byte) {
		var decoded interoppb.StateHandoff
		if err := proto.Unmarshal(msg, &decoded); err == nil {
			handler(&decoded)
		}
	})
}

// ListenForPings listens for HealthPings and sends HealthAcks
func (p *Protocol) ListenForPings(ctx context.Context) (kairos.Subscription, error) {
	return p.mesh.Subscribe(ctx, "system:health_ping", func(msg []byte) {
		var decoded interoppb.HealthPing
		if err := proto.Unmarshal(msg, &decoded); err == nil {
			ack := &interoppb.HealthAck{
				SourceNodeId: p.nodeID,
				TargetNodeId: decoded.SourceNodeId,
				TimestampMs:  time.Now().UnixMilli(),
			}

			if buf, err := proto.Marshal(ack); err == nil {
				topic := fmt.Sprintf("system:health_ack:%s", decoded.SourceNodeId)
				go func() {
					retries := 0
					delayMs := 50
					for retries < 5 {
						if err := p.mesh.Publish(context.Background(), topic, buf); err == nil {
							break
						}
						retries++
						time.Sleep(time.Duration(delayMs) * time.Millisecond)
						delayMs *= 2
					}
				}()
			}
		}
	})
}

// DispatchJob dispatches a background job and waits for acknowledgment
func (p *Protocol) DispatchJob(ctx context.Context, jobID, tenantID, actionName string, payload []byte, timeoutMs int64) (bool, error) {
	ackTopic := fmt.Sprintf("system:job_ack:%s", jobID)
	receivedCh := make(chan struct{}, 1)

	sub, err := p.mesh.Subscribe(ctx, ackTopic, func(msg []byte) {
		select {
		case receivedCh <- struct{}{}:
		default:
		}
	})
	if err != nil {
		return false, err
	}
	defer sub.Unsubscribe()

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

	topic := fmt.Sprintf("system:job_dispatch:%s", tenantID)

	retries := 0
	delayMs := 100
	for {
		if err := p.mesh.Publish(ctx, topic, buf); err == nil {
			break
		}

		if retries >= 5 {
			return false, fmt.Errorf("failed to publish job dispatch after retries: %w", err)
		}

		time.Sleep(time.Duration(delayMs) * time.Millisecond)
		delayMs *= 2
		retries++
	}

	select {
	case <-receivedCh:
		return true, nil
	case <-time.After(time.Duration(timeoutMs) * time.Millisecond):
		return false, nil
	}
}

// ListenForJobs listens for job dispatches and acknowledges them
func (p *Protocol) ListenForJobs(ctx context.Context, tenantID string) (kairos.Subscription, error) {
	topic := fmt.Sprintf("system:job_dispatch:%s", tenantID)
	return p.mesh.Subscribe(ctx, topic, func(msg []byte) {
		var decoded interoppb.JobDispatch
		if err := proto.Unmarshal(msg, &decoded); err == nil {
			ack := &interoppb.JobAck{
				JobId:       decoded.JobId,
				NodeId:      p.nodeID,
				TimestampMs: time.Now().UnixMilli(),
			}

			if buf, err := proto.Marshal(ack); err == nil {
				ackTopic := fmt.Sprintf("system:job_ack:%s", decoded.JobId)
				go func() {
					retries := 0
					delayMs := 50
					for retries < 5 {
						if err := p.mesh.Publish(context.Background(), ackTopic, buf); err == nil {
							break
						}
						retries++
						time.Sleep(time.Duration(delayMs) * time.Millisecond)
						delayMs *= 2
					}
				}()
			}
		}
	})
}

// ReportJobStatus reports job status back to the main server
func (p *Protocol) ReportJobStatus(ctx context.Context, jobID, tenantID, status string, details []byte) error {
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

	topic := fmt.Sprintf("system:job_status:%s", jobID)

	retries := 0
	delayMs := 100
	for {
		if err := p.mesh.Publish(ctx, topic, buf); err == nil {
			return nil
		}

		if retries >= 5 {
			return fmt.Errorf("failed to publish job status update after retries: %w", err)
		}

		time.Sleep(time.Duration(delayMs) * time.Millisecond)
		delayMs *= 2
		retries++
	}
}

// ListenForJobStatus listens for job status updates for a specific job
func (p *Protocol) ListenForJobStatus(ctx context.Context, jobID string, handler func(*interoppb.JobStatusUpdate)) (kairos.Subscription, error) {
	topic := fmt.Sprintf("system:job_status:%s", jobID)
	return p.mesh.Subscribe(ctx, topic, func(msg []byte) {
		var decoded interoppb.JobStatusUpdate
		if err := proto.Unmarshal(msg, &decoded); err == nil {
			handler(&decoded)
		}
	})
}
