package interop

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/proto/interop"
	"google.golang.org/protobuf/proto"
)

// HealthMonitor tracks the cross-mode health of agents in the swarm.
type HealthMonitor interface {
	// Ping sends a health probe and waits for a response.
	Ping(ctx context.Context, targetAgentID string) (*interoppb.HealthStatus, error)

	// StartResponder starts listening for health probes and answering them.
	StartResponder(ctx context.Context, agentID string) error
}

type healthMonitorImpl struct {
	mesh      TeammateMesh
	probeChan string
	respChan  string

	mu        sync.RWMutex
	pending   map[string]chan *interoppb.HealthStatus
}

// NewHealthMonitor creates a new health monitor using the TeammateMesh.
func NewHealthMonitor(mesh TeammateMesh) (HealthMonitor, error) {
	hm := &healthMonitorImpl{
		mesh:      mesh,
		probeChan: "health.probes",
		respChan:  "health.responses",
		pending:   make(map[string]chan *interoppb.HealthStatus),
	}

	go hm.listenForResponses()

	return hm, nil
}

func (hm *healthMonitorImpl) listenForResponses() {
	ctx := context.Background()
	sub, err := hm.mesh.Subscribe(ctx, hm.respChan)
	if err != nil {
		return
	}

	for msg := range sub {
		var status interoppb.HealthStatus
		if err := proto.Unmarshal(msg, &status); err != nil {
			continue
		}

		hm.mu.RLock()
		ch, ok := hm.pending[status.ProbeId]
		hm.mu.RUnlock()

		if ok {
			select {
			case ch <- &status:
			default:
			}
		}
	}
}

func (hm *healthMonitorImpl) Ping(ctx context.Context, targetAgentID string) (*interoppb.HealthStatus, error) {
	probe := &interoppb.HealthProbe{
		ProbeId:   fmt.Sprintf("probe-%d", time.Now().UnixNano()),
		Timestamp: time.Now().Unix(),
	}

	data, err := proto.Marshal(probe)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal probe: %w", err)
	}

	respCh := make(chan *interoppb.HealthStatus, 1)
	hm.mu.Lock()
	hm.pending[probe.ProbeId] = respCh
	hm.mu.Unlock()

	defer func() {
		hm.mu.Lock()
		delete(hm.pending, probe.ProbeId)
		hm.mu.Unlock()
	}()

	// Target channel includes agent ID for directed pings if possible,
	// but here we broadcast and expect the specific agent to respond.
	targetChan := fmt.Sprintf("%s.%s", hm.probeChan, targetAgentID)
	if err := hm.mesh.Publish(ctx, targetChan, data); err != nil {
		return nil, fmt.Errorf("failed to publish probe: %w", err)
	}

	select {
	case status := <-respCh:
		return status, nil
	case <-time.After(2 * time.Second):
		return nil, fmt.Errorf("ping timeout to agent %s", targetAgentID)
	case <-ctx.Done():
		return nil, ctx.Err()
	}
}

func (hm *healthMonitorImpl) StartResponder(ctx context.Context, agentID string) error {
	targetChan := fmt.Sprintf("%s.%s", hm.probeChan, agentID)
	sub, err := hm.mesh.Subscribe(ctx, targetChan)
	if err != nil {
		return fmt.Errorf("failed to subscribe to probes: %w", err)
	}

	go func() {
		for {
			select {
			case msg, ok := <-sub:
				if !ok {
					return
				}

				var probe interoppb.HealthProbe
				if err := proto.Unmarshal(msg, &probe); err != nil {
					continue
				}

				status := &interoppb.HealthStatus{
					ProbeId:   probe.ProbeId,
					AgentId:   agentID,
					Timestamp: time.Now().Unix(),
					Status:    "healthy",
				}

				if data, err := proto.Marshal(status); err == nil {
					_ = hm.mesh.Publish(ctx, hm.respChan, data)
				}
			case <-ctx.Done():
				return
			}
		}
	}()

	return nil
}
