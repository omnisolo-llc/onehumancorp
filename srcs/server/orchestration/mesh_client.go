package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/orchestration/mesh"

	_ "github.com/mattn/go-sqlite3"
	pb "github.com/onehumancorp/mono/srcs/proto"
	agentgrpc "github.com/onehumancorp/mono/srcs/server/agents/builtin/grpc"
	"log/slog"
	"time"
)

type MeshClient interface {
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (mesh.Subscription, error)
	StartHeartbeat(ctx context.Context, caps pb.AgentCapabilities) error
}

type StandardMeshClient struct {
	transport MeshTransport
}

func NewStandardMeshClient(transport MeshTransport) *StandardMeshClient {
	return &StandardMeshClient{
		transport: transport,
	}
}

func (c *StandardMeshClient) Publish(ctx context.Context, topic string, payload []byte) error {
	return c.transport.BroadcastMeshEvent(ctx, topic, payload)
}

func (c *StandardMeshClient) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (mesh.Subscription, error) {
	return nil, fmt.Errorf("StandardMeshClient.Subscribe not fully implemented")
}

func (c *StandardMeshClient) StartHeartbeat(ctx context.Context, caps pb.AgentCapabilities) error {
	go func() {
		ticker := time.NewTicker(30 * time.Second)
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				if err := c.transport.AdvertiseCapabilities(ctx, caps); err != nil {
					slog.Error("Failed to advertise capabilities in heartbeat", "error", err)
				}
			}
		}
	}()
	return nil
}

// TriggerBurst handles the elastic swarm bursting handoff.
var dbDSN = "file:ohc.db?mode=rw"

func TriggerBurst(ctx context.Context, missionID string) error {
	db, err := sql.Open("sqlite3", dbDSN)
	if err != nil {
		return fmt.Errorf("failed to open db: %w", err)
	}
	defer db.Close()

	// 1. Serialize state
	var payload string
	err = db.QueryRowContext(ctx, "SELECT payload FROM agent_missions WHERE id = ?", missionID).Scan(&payload)
	if err != nil && err != sql.ErrNoRows {
		return fmt.Errorf("failed to fetch mission payload: %w", err)
	}

	// 2. Update SQLite status to BURSTING
	_, err = db.ExecContext(ctx, "UPDATE agent_missions SET status = 'BURSTING' WHERE id = ?", missionID)
	if err != nil {
		return fmt.Errorf("failed to update mission status: %w", err)
	}

	// 3. Transmit payload securely to Cloud API via gRPC
	client, err := agentgrpc.NewClient(agentgrpc.DefaultAddress, agentgrpc.ClientOptions{})
	if err != nil {
		return fmt.Errorf("failed to create grpc client: %w", err)
	}
	defer client.Close()

	err = client.TransmitMissionPayload(ctx, []byte(payload))
	if err != nil {
		return fmt.Errorf("failed to transmit mission payload: %w", err)
	}

	return nil
}
