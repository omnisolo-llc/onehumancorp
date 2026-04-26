package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	_ "github.com/mattn/go-sqlite3"
	agentgrpc "github.com/onehumancorp/mono/src/server_old/agents/grpc"
	"github.com/onehumancorp/mono/src/server/orchestration/mesh"
)

type MeshClient interface {
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (mesh.Subscription, error)
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
	if err != nil {
		if err == sql.ErrNoRows {
			return fmt.Errorf("mission payload not found for id: %s", missionID)
		}
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
