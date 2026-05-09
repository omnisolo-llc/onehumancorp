package interop

import (
	"context"
	"sync/atomic"
	"testing"
	"time"

	"onehumancorp/srcs/server/orchestration"
	pb "onehumancorp/srcs/server/pb/interop"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"github.com/golang/protobuf/proto"
)

func TestInteropProtocol_Handoff(t *testing.T) {
	mesh := orchestration.NewLocalTeammateMesh()
	lock := NewMemoryLock()
	protocol := NewInteropProtocol(mesh, lock, "node-1")
	ctx := context.Background()

	missionID := "mission_1"
	tenantID := "tenant_1"
	payload := []byte("state_data")

	var received int32
	err := protocol.ListenForStateHandoff(ctx, func(msg *pb.StateHandoff) {
		if msg.MissionId == missionID && string(msg.StateSnapshotJson) == "state_data" {
			atomic.AddInt32(&received, 1)
		}
	})
	require.NoError(t, err)

	err = protocol.Handoff(ctx, missionID, tenantID, payload)
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond)

	assert.Equal(t, int32(1), atomic.LoadInt32(&received))

	// Test idempotency
	err = protocol.Handoff(ctx, missionID, tenantID, payload)
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond)

	// Should still be 1
	assert.Equal(t, int32(1), atomic.LoadInt32(&received))
}

func TestInteropProtocol_HealthPing(t *testing.T) {
	mesh := orchestration.NewLocalTeammateMesh()
	lock := NewMemoryLock()
	protocolListener := NewInteropProtocol(mesh, lock, "listener-node")
	protocolSender := NewInteropProtocol(mesh, lock, "sender-node")

	ctx := context.Background()

	err := protocolListener.ListenForPings(ctx)
	require.NoError(t, err)

	// Wait for subscription to propagate
	time.Sleep(50 * time.Millisecond)

	ok, err := protocolSender.CheckHealth(ctx, 1000)
	require.NoError(t, err)
	assert.True(t, ok)
}

func TestInteropProtocol_DispatchJob(t *testing.T) {
	mesh := orchestration.NewLocalTeammateMesh()
	lock := NewMemoryLock()
	protocolAgent := NewInteropProtocol(mesh, lock, "agent-node")
	protocolServer := NewInteropProtocol(mesh, lock, "server-node")

	ctx := context.Background()

	var received int32
	err := protocolAgent.ListenForJobs(ctx, "tenant-a", func(msg *pb.JobDispatch) {
		if msg.JobId == "job-123" {
			atomic.AddInt32(&received, 1)

			// Agent sends back an ack
			ack := &pb.JobAck{
				JobId:       msg.JobId,
				NodeId:      "agent-node",
				TimestampMs: time.Now().UnixMilli(),
			}
			buf, _ := proto.Marshal(ack)
			_ = mesh.Publish(context.Background(), "system:job_ack:job-123", buf)
		}
	})
	require.NoError(t, err)

	time.Sleep(50 * time.Millisecond)

	ok, err := protocolServer.DispatchJob(ctx, "job-123", "tenant-a", "do_work", []byte("payload"), 1000)
	require.NoError(t, err)
	assert.True(t, ok)

	assert.Equal(t, int32(1), atomic.LoadInt32(&received))
}

func TestInteropProtocol_JobStatus(t *testing.T) {
	mesh := orchestration.NewLocalTeammateMesh()
	lock := NewMemoryLock()
	protocolServer := NewInteropProtocol(mesh, lock, "server-node")
	protocolAgent := NewInteropProtocol(mesh, lock, "agent-node")

	ctx := context.Background()

	var received int32
	err := protocolServer.ListenForJobStatus(ctx, "job-456", func(msg *pb.JobStatusUpdate) {
		if msg.Status == "COMPLETED" {
			atomic.AddInt32(&received, 1)
		}
	})
	require.NoError(t, err)

	time.Sleep(50 * time.Millisecond)

	err = protocolAgent.ReportJobStatus(ctx, "job-456", "tenant-a", "COMPLETED", []byte("result"))
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond)

	assert.Equal(t, int32(1), atomic.LoadInt32(&received))
}

func TestInteropProtocol_LockTimeout(t *testing.T) {
	mesh := orchestration.NewLocalTeammateMesh()
	lock := NewMemoryLock()
	protocol := NewInteropProtocol(mesh, lock, "node-1")
	ctx := context.Background()

	// Acquire the lock manually to simulate another node holding it
	lockResource := "handoff:mission_2"
	acquired, err := lock.AcquireLock(ctx, lockResource, "other-node", 10)
	require.NoError(t, err)
	assert.True(t, acquired)

	// Handoff should eventually time out
	err = protocol.Handoff(ctx, "mission_2", "tenant_2", []byte("data"))
	require.Error(t, err)
	assert.Contains(t, err.Error(), "Timeout waiting for lock")
}
