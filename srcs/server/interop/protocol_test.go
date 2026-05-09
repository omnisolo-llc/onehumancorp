package interop

import (
	"context"
	"sync/atomic"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"google.golang.org/protobuf/proto"
	"onehumancorp/srcs/server/orchestration/kairos"
	"onehumancorp/srcs/server/pb/interoppb"
)

func TestInteropHandoffMemory(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocol := NewProtocol(mesh, "node1")

	var received int32
	ctx := context.Background()

	sub, err := protocol.ListenForStateHandoff(ctx, func(msg *interoppb.StateHandoff) {
		if msg.MissionId == "mission_1" {
			atomic.StoreInt32(&received, 1)
		}
	})
	require.NoError(t, err)
	defer sub.Unsubscribe()

	err = protocol.Handoff(ctx, "mission_1", "tenant_1", []byte{1, 2, 3})
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond)
	assert.Equal(t, int32(1), atomic.LoadInt32(&received))
}

func TestInteropDispatchJobTimeout(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocolServer := NewProtocol(mesh, "server")

	ctx := context.Background()
	isAcked, err := protocolServer.DispatchJob(ctx, "job_timeout", "tenant_a", "do_work", []byte{42}, 100)
	require.NoError(t, err)
	assert.False(t, isAcked)
}

func TestInteropListenForPings(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocolListener := NewProtocol(mesh, "listener_node")

	ctx := context.Background()
	sub, err := protocolListener.ListenForPings(ctx)
	require.NoError(t, err)
	defer sub.Unsubscribe()

	var received int32
	ackTopic := "system:health_ack:sender_node"

	ackSub, err := mesh.Subscribe(ctx, ackTopic, func(msg []byte) {
		atomic.StoreInt32(&received, 1)
	})
	require.NoError(t, err)
	defer ackSub.Unsubscribe()

	ping := &interoppb.HealthPing{
		CurrentMode:   interoppb.DeploymentMode_MODE_UNSPECIFIED,
		TimestampMs:   time.Now().UnixMilli(),
		SourceNodeId:  "sender_node",
	}

	buf, err := proto.Marshal(ping)
	require.NoError(t, err)

	err = mesh.Publish(ctx, "system:health_ping", buf)
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond)
	assert.Equal(t, int32(1), atomic.LoadInt32(&received))
}

func TestInteropListenForJobs(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocolListener := NewProtocol(mesh, "listener_node")

	ctx := context.Background()
	sub, err := protocolListener.ListenForJobs(ctx, "tenant_x")
	require.NoError(t, err)
	defer sub.Unsubscribe()

	var received int32
	ackTopic := "system:job_ack:job_123"

	ackSub, err := mesh.Subscribe(ctx, ackTopic, func(msg []byte) {
		atomic.StoreInt32(&received, 1)
	})
	require.NoError(t, err)
	defer ackSub.Unsubscribe()

	dispatch := &interoppb.JobDispatch{
		JobId:       "job_123",
		TenantId:    "tenant_x",
		ActionName:  "test_action",
		PayloadJson: []byte{1, 2, 3},
		TimestampMs: time.Now().UnixMilli(),
	}

	buf, err := proto.Marshal(dispatch)
	require.NoError(t, err)

	err = mesh.Publish(ctx, "system:job_dispatch:tenant_x", buf)
	require.NoError(t, err)

	time.Sleep(200 * time.Millisecond)
	assert.Equal(t, int32(1), atomic.LoadInt32(&received))
}

func TestInteropHandoffLockDeadlockPrevention(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocol1 := NewProtocol(mesh, "node1")

	ctx := context.Background()
	ok, err := mesh.AcquireLock(ctx, "handoff:mission_locked", 10*time.Second)
	require.NoError(t, err)
	require.True(t, ok)

	err = protocol1.Handoff(ctx, "mission_locked", "tenant_1", []byte{1, 2, 3})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "timeout waiting for lock")

	err = mesh.ReleaseLock(ctx, "handoff:mission_locked")
	require.NoError(t, err)
}

func TestInteropJobStatusReporting(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocolServer := NewProtocol(mesh, "server")
	protocolAgent := NewProtocol(mesh, "agent")

	var received int32
	ctx := context.Background()

	sub, err := protocolServer.ListenForJobStatus(ctx, "job_status_123", func(update *interoppb.JobStatusUpdate) {
		if update.JobId == "job_status_123" && update.Status == "COMPLETED" {
			atomic.StoreInt32(&received, 1)
		}
	})
	require.NoError(t, err)
	defer sub.Unsubscribe()

	err = protocolAgent.ReportJobStatus(ctx, "job_status_123", "tenant_a", "COMPLETED", []byte{1, 2, 3})
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond)
	assert.Equal(t, int32(1), atomic.LoadInt32(&received))
}

type FailingMesh struct {
	*kairos.LocalTeammateMesh
	FailuresLeft int32
}

func (m *FailingMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	if atomic.AddInt32(&m.FailuresLeft, -1) >= 0 {
		return assert.AnError
	}
	return m.LocalTeammateMesh.Publish(ctx, topic, payload)
}

func TestInteropDispatchJobRetrySuccess(t *testing.T) {
	mesh := &FailingMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh(), FailuresLeft: 3}
	protocol := NewProtocol(mesh, "server")

	ctx := context.Background()
	// Mock bus doesn't reply with ACK, so it will timeout and return false, but won't fail the dispatch completely.
	isAcked, err := protocol.DispatchJob(ctx, "job_retry_1", "tenant_a", "do_work", []byte{}, 10)
	require.NoError(t, err)
	assert.False(t, isAcked)
}

func TestInteropDispatchJobRetryFailure(t *testing.T) {
	mesh := &FailingMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh(), FailuresLeft: 10}
	protocol := NewProtocol(mesh, "server")

	ctx := context.Background()
	_, err := protocol.DispatchJob(ctx, "job_retry_2", "tenant_a", "do_work", []byte{}, 10)
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to publish job dispatch after retries")
}

func TestInteropHandoffRetrySuccess(t *testing.T) {
	mesh := &FailingMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh(), FailuresLeft: 3}
	protocol := NewProtocol(mesh, "node1")

	ctx := context.Background()
	err := protocol.Handoff(ctx, "mission_retry_1", "tenant_1", []byte{1, 2, 3})
	require.NoError(t, err)
}

func TestInteropHandoffRetryFailure(t *testing.T) {
	mesh := &FailingMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh(), FailuresLeft: 10}
	protocol := NewProtocol(mesh, "node1")

	ctx := context.Background()
	err := protocol.Handoff(ctx, "mission_retry_2", "tenant_1", []byte{1, 2, 3})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to publish state handoff after retries")
}

func TestInteropJobStatusReportingRetrySuccess(t *testing.T) {
	mesh := &FailingMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh(), FailuresLeft: 3}
	protocol := NewProtocol(mesh, "agent")

	ctx := context.Background()
	err := protocol.ReportJobStatus(ctx, "job_retry_1", "tenant_a", "FAILED", []byte{})
	require.NoError(t, err)
}

func TestInteropJobStatusReportingRetryFailure(t *testing.T) {
	mesh := &FailingMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh(), FailuresLeft: 10}
	protocol := NewProtocol(mesh, "agent")

	ctx := context.Background()
	err := protocol.ReportJobStatus(ctx, "job_retry_2", "tenant_a", "FAILED", []byte{})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to publish job status update after retries")
}

type FailingMarshalMesh struct {
	*kairos.LocalTeammateMesh
}
func (m *FailingMarshalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
    if key == "handoff:processed:mission_1" {
        return false, assert.AnError
    }
    return m.LocalTeammateMesh.AcquireLock(ctx, key, ttl)
}

func TestInteropHandoffIdempotencyLockFailure(t *testing.T) {
	mesh := &FailingMarshalMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh()}
	protocol := NewProtocol(mesh, "node1")

	ctx := context.Background()
	err := protocol.Handoff(ctx, "mission_1", "tenant_1", []byte{1, 2, 3})
	require.Error(t, err)
}

type SubscriberFailingMesh struct {
	*kairos.LocalTeammateMesh
}
func (m *SubscriberFailingMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (kairos.Subscription, error) {
    return nil, assert.AnError
}

func TestInteropDispatchJobSubscribeFailure(t *testing.T) {
	mesh := &SubscriberFailingMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh()}
	protocol := NewProtocol(mesh, "server")

	ctx := context.Background()
	isAcked, err := protocol.DispatchJob(ctx, "job_sub_fail", "tenant_a", "do_work", []byte{}, 10)
	require.Error(t, err)
	assert.False(t, isAcked)
}

func TestInteropListenForPingsPublishRetry(t *testing.T) {
	mesh := &FailingMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh(), FailuresLeft: 3}
	protocolListener := NewProtocol(mesh, "listener_node")

	ctx := context.Background()
	sub, err := protocolListener.ListenForPings(ctx)
	require.NoError(t, err)
	defer sub.Unsubscribe()

	var received int32
	ackTopic := "system:health_ack:sender_node"

	ackSub, err := mesh.Subscribe(ctx, ackTopic, func(msg []byte) {
		atomic.StoreInt32(&received, 1)
	})
	require.NoError(t, err)
	defer ackSub.Unsubscribe()

	ping := &interoppb.HealthPing{
		CurrentMode:   interoppb.DeploymentMode_MODE_UNSPECIFIED,
		TimestampMs:   time.Now().UnixMilli(),
		SourceNodeId:  "sender_node",
	}

	buf, err := proto.Marshal(ping)
	require.NoError(t, err)

	err = mesh.LocalTeammateMesh.Publish(ctx, "system:health_ping", buf)
	require.NoError(t, err)

	time.Sleep(1000 * time.Millisecond)
	assert.Equal(t, int32(1), atomic.LoadInt32(&received))
}

func TestInteropListenForJobsPublishRetry(t *testing.T) {
	mesh := &FailingMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh(), FailuresLeft: 3}
	protocolListener := NewProtocol(mesh, "listener_node")

	ctx := context.Background()
	sub, err := protocolListener.ListenForJobs(ctx, "tenant_x")
	require.NoError(t, err)
	defer sub.Unsubscribe()

	var received int32
	ackTopic := "system:job_ack:job_123"

	ackSub, err := mesh.Subscribe(ctx, ackTopic, func(msg []byte) {
		atomic.StoreInt32(&received, 1)
	})
	require.NoError(t, err)
	defer ackSub.Unsubscribe()

	dispatch := &interoppb.JobDispatch{
		JobId:       "job_123",
		TenantId:    "tenant_x",
		ActionName:  "test_action",
		PayloadJson: []byte{1, 2, 3},
		TimestampMs: time.Now().UnixMilli(),
	}

	buf, err := proto.Marshal(dispatch)
	require.NoError(t, err)

	err = mesh.LocalTeammateMesh.Publish(ctx, "system:job_dispatch:tenant_x", buf)
	require.NoError(t, err)

	time.Sleep(1000 * time.Millisecond)
	assert.Equal(t, int32(1), atomic.LoadInt32(&received))
}

type TimeoutAcquireMesh struct {
	*kairos.LocalTeammateMesh
}

func (m *TimeoutAcquireMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
    if key == "handoff:mission_2" {
        return false, nil
    }
    return m.LocalTeammateMesh.AcquireLock(ctx, key, ttl)
}

func TestInteropHandoffAcquireTimeout(t *testing.T) {
	mesh := &TimeoutAcquireMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh()}
	protocol := NewProtocol(mesh, "node1")

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()
	err := protocol.Handoff(ctx, "mission_2", "tenant_1", []byte{1, 2, 3})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "timeout waiting for lock")
}

func TestInteropListenForPingsUnmarshalError(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocolListener := NewProtocol(mesh, "listener_node")

	ctx := context.Background()
	sub, err := protocolListener.ListenForPings(ctx)
	require.NoError(t, err)
	defer sub.Unsubscribe()

	err = mesh.Publish(ctx, "system:health_ping", []byte("invalid_proto"))
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond) // Should not crash
}

func TestInteropListenForJobsUnmarshalError(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocolListener := NewProtocol(mesh, "listener_node")

	ctx := context.Background()
	sub, err := protocolListener.ListenForJobs(ctx, "tenant_x")
	require.NoError(t, err)
	defer sub.Unsubscribe()

	err = mesh.Publish(ctx, "system:job_dispatch:tenant_x", []byte("invalid_proto"))
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond) // Should not crash
}

func TestInteropListenForStateHandoffUnmarshalError(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocol := NewProtocol(mesh, "node1")

	ctx := context.Background()
	sub, err := protocol.ListenForStateHandoff(ctx, func(msg *interoppb.StateHandoff) {})
	require.NoError(t, err)
	defer sub.Unsubscribe()

	err = mesh.Publish(ctx, "system:state_handoff", []byte("invalid_proto"))
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond) // Should not crash
}

func TestInteropListenForJobStatusUnmarshalError(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocol := NewProtocol(mesh, "server")

	ctx := context.Background()
	sub, err := protocol.ListenForJobStatus(ctx, "job_status_123", func(update *interoppb.JobStatusUpdate) {})
	require.NoError(t, err)
	defer sub.Unsubscribe()

	err = mesh.Publish(ctx, "system:job_status:job_status_123", []byte("invalid_proto"))
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond) // Should not crash
}

type FailingPublishMesh struct {
	*kairos.LocalTeammateMesh
}

func (m *FailingPublishMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return assert.AnError
}

func TestInteropHandoffPublishFail(t *testing.T) {
	mesh := &FailingPublishMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh()}
	protocol := NewProtocol(mesh, "node1")

	ctx := context.Background()
	err := protocol.Handoff(ctx, "mission_1", "tenant_1", []byte{1, 2, 3})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to publish state handoff after retries")
}

func TestInteropDispatchJobPublishFail(t *testing.T) {
	mesh := &FailingPublishMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh()}
	protocolServer := NewProtocol(mesh, "server")

	ctx := context.Background()
	isAcked, err := protocolServer.DispatchJob(ctx, "job_timeout", "tenant_a", "do_work", []byte{42}, 100)
	require.Error(t, err)
	assert.False(t, isAcked)
	assert.Contains(t, err.Error(), "failed to publish job dispatch after retries")
}

func TestInteropReportJobStatusPublishFail(t *testing.T) {
	mesh := &FailingPublishMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh()}
	protocolAgent := NewProtocol(mesh, "agent")

	ctx := context.Background()
	err := protocolAgent.ReportJobStatus(ctx, "job_status_123", "tenant_a", "COMPLETED", []byte{1, 2, 3})
	require.Error(t, err)
	assert.Contains(t, err.Error(), "failed to publish job status update after retries")
}


func TestInteropHandoffIdempotencyLockAlreadyProcessed(t *testing.T) {
	mesh := kairos.NewLocalTeammateMesh()
	protocol := NewProtocol(mesh, "node1")

	ctx := context.Background()

	// Simulate already processed
	ok, err := mesh.AcquireLock(ctx, "handoff:processed:mission_already_processed", 3600*time.Second)
	require.NoError(t, err)
	require.True(t, ok)

	err = protocol.Handoff(ctx, "mission_already_processed", "tenant_1", []byte{1, 2, 3})
	require.NoError(t, err) // Should return nil early
}

type SubscribeFailMesh struct {
	*kairos.LocalTeammateMesh
}

func (m *SubscribeFailMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (kairos.Subscription, error) {
    if topic == "system:job_ack:job_timeout_sub_fail" {
        return nil, assert.AnError
    }
    return m.LocalTeammateMesh.Subscribe(ctx, topic, handler)
}

func TestInteropDispatchJobSubFail(t *testing.T) {
	mesh := &SubscribeFailMesh{LocalTeammateMesh: kairos.NewLocalTeammateMesh()}
	protocolServer := NewProtocol(mesh, "server")

	ctx := context.Background()
	isAcked, err := protocolServer.DispatchJob(ctx, "job_timeout_sub_fail", "tenant_a", "do_work", []byte{42}, 100)
	require.Error(t, err)
	assert.False(t, isAcked)
}
