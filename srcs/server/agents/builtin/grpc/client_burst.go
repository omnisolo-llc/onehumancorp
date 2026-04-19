package agentgrpc

import (
	"context"
	"fmt"
	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
)

// TransmitMissionPayload transmits the agent_missions payload to the cloud API.
func (c *Client) TransmitMissionPayload(ctx context.Context, payload []byte) error {
	req := &agentservicepb.RunTaskRequest{
		TaskId: "burst",
		Task:   string(payload),
		Model:  "cloud-burst-model",
	}

	err := c.RunTask(ctx, req, func(evt *agentservicepb.RunTaskEvent) {
		// Callback: Results are streamed back to the local client
		_ = evt
	})
	if err != nil {
		return fmt.Errorf("grpc transmission failed: %w", err)
	}

	return nil
}
