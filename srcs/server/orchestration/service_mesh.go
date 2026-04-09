package orchestration

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/telemetry"

)

// AdvertiseCapabilities advertises an agent's capabilities to the mesh
func (s *HubServiceServer) AdvertiseCapabilities(ctx context.Context, req *pb.AgentCapabilities) (*pb.PublishMessageResponse, error) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(ctx, "AdvertiseCapabilities", time.Since(start)) }()

	if req.GetAgentId() == "" {
		return nil, fmt.Errorf("agent_id is required")
	}

	cn := s.hub.CentrifugeNode()
	if cn == nil {
		return nil, fmt.Errorf("CentrifugeNode is nil")
	}

	if cn.meshTransport == nil {
		return nil, fmt.Errorf("meshTransport is not configured")
	}

	if err := cn.meshTransport.AdvertiseCapabilities(ctx, *req); err != nil {
		slog.Error("Failed to advertise capabilities", "error", err, "agent_id", req.GetAgentId())
		return nil, fmt.Errorf("failed to broadcast capabilities: %w", err)
	}

	telemetry.RecordMeshBroadcast(ctx, "capabilities")

	return &pb.PublishMessageResponse{Success: true}, nil
}

// DiscoverAgents streams known agent capabilities from the mesh

// StreamMeshEvents streams real-time events from the mesh
