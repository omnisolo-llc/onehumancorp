package interop

import (
	"context"

	meshpb "github.com/onehumancorp/mono/srcs/proto/mesh"
	"google.golang.org/protobuf/proto"
)

// MeshTransport defines the generic pub/sub interface
type MeshTransport interface {
	Publish(topic string, data []byte) error
	Subscribe(topic string) (<-chan []byte, error)
}

// MeshInterop Layer ensures main server and builtin agent microservice can communicate
// via Protobuf messages over the Teammate Mesh transport, abstracting away whether
// it's running in Cloud (Redis) or Standalone (Memory IPC) mode.
type MeshInterop struct {
	transport MeshTransport
}

func NewMeshInterop(transport MeshTransport) *MeshInterop {
	return &MeshInterop{
		transport: transport,
	}
}

// DispatchJob sends a job dispatch message to the builtin agent
func (m *MeshInterop) DispatchJob(ctx context.Context, dispatch *meshpb.MeshJobDispatch) error {
	data, err := proto.Marshal(dispatch)
	if err != nil {
		return err
	}
	return m.transport.Publish("mesh:jobs:dispatch", data)
}

// SubscribeJobStatus listens for job status updates from the builtin agent
func (m *MeshInterop) SubscribeJobStatus(ctx context.Context) (<-chan *meshpb.MeshJobStatus, error) {
	ch, err := m.transport.Subscribe("mesh:jobs:status")
	if err != nil {
		return nil, err
	}

	statusCh := make(chan *meshpb.MeshJobStatus, 100)

	go func() {
		for data := range ch {
			var status meshpb.MeshJobStatus
			if err := proto.Unmarshal(data, &status); err == nil {
				statusCh <- &status
			}
		}
		close(statusCh)
	}()

	return statusCh, nil
}

// SyncContext sends context (AI memory/embeddings) between modes
func (m *MeshInterop) SyncContext(ctx context.Context, sync *meshpb.MeshContextSync) error {
	data, err := proto.Marshal(sync)
	if err != nil {
		return err
	}
	return m.transport.Publish("mesh:context:sync", data)
}

// HandoffState triggers a state handoff when switching between Cloud/Standalone
func (m *MeshInterop) HandoffState(ctx context.Context, handoff *meshpb.MeshHandoff) error {
	data, err := proto.Marshal(handoff)
	if err != nil {
		return err
	}
	return m.transport.Publish("mesh:state:handoff", data)
}

// SubscribeContextSync listens for context sync messages
func (m *MeshInterop) SubscribeContextSync(ctx context.Context) (<-chan *meshpb.MeshContextSync, error) {
	ch, err := m.transport.Subscribe("mesh:context:sync")
	if err != nil {
		return nil, err
	}

	syncCh := make(chan *meshpb.MeshContextSync, 100)

	go func() {
		for data := range ch {
			var syncMsg meshpb.MeshContextSync
			if err := proto.Unmarshal(data, &syncMsg); err == nil {
				syncCh <- &syncMsg
			}
		}
		close(syncCh)
	}()

	return syncCh, nil
}

// SubscribeHandoff listens for state handoff events
func (m *MeshInterop) SubscribeHandoff(ctx context.Context) (<-chan *meshpb.MeshHandoff, error) {
	ch, err := m.transport.Subscribe("mesh:state:handoff")
	if err != nil {
		return nil, err
	}

	handoffCh := make(chan *meshpb.MeshHandoff, 100)

	go func() {
		for data := range ch {
			var handoffMsg meshpb.MeshHandoff
			if err := proto.Unmarshal(data, &handoffMsg); err == nil {
				handoffCh <- &handoffMsg
			}
		}
		close(handoffCh)
	}()

	return handoffCh, nil
}
