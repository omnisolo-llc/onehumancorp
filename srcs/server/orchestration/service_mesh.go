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
	telemetry.RecordMeshMessageThroughput(ctx, len(req.String()), "capabilities")

	return &pb.PublishMessageResponse{Success: true}, nil
}

// DiscoverAgents streams known agent capabilities from the mesh
func (s *HubServiceServer) DiscoverAgents(req *pb.Query, stream pb.HubService_DiscoverAgentsServer) error {
	ctx := stream.Context()

	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(ctx, "DiscoverAgents", time.Since(start)) }()

	cn := s.hub.CentrifugeNode()
	if cn == nil {
		return fmt.Errorf("CentrifugeNode is nil")
	}

	if cn.meshTransport == nil {
		return fmt.Errorf("meshTransport is not configured")
	}

	capsChan, err := cn.meshTransport.SubscribeCapabilities(ctx)
	if err != nil {
		return fmt.Errorf("failed to subscribe to capabilities: %w", err)
	}

	for {
		select {
		case <-ctx.Done():
			return nil
		case caps, ok := <-capsChan:
			if !ok {
				return nil
			}

			// Optional: Apply filtering based on req.GetFilter() here if implemented later

			if err := stream.Send(&caps); err != nil {
				return err
			}

			telemetry.RecordMeshMessageThroughput(ctx, len(caps.String()), "capabilities")
		}
	}
}

// StreamMeshEvents streams real-time events from the mesh
func (s *HubServiceServer) StreamMeshEvents(req *pb.EventStreamRequest, stream pb.HubService_StreamMeshEventsServer) error {
	ctx := stream.Context()

	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(ctx, "StreamMeshEvents", time.Since(start)) }()

	if req.GetTopic() == "" {
		return fmt.Errorf("topic is required")
	}

	cn := s.hub.CentrifugeNode()
	if cn == nil {
		return fmt.Errorf("CentrifugeNode is nil")
	}

	if cn.meshTransport == nil {
		return fmt.Errorf("meshTransport is not configured")
	}

	eventsChan, err := cn.meshTransport.SubscribeMeshEvents(ctx, req.GetTopic())
	if err != nil {
		return fmt.Errorf("failed to subscribe to mesh events: %w", err)
	}

	for {
		select {
		case <-ctx.Done():
			return nil
		case payload, ok := <-eventsChan:
			if !ok {
				return nil
			}

			event := &pb.MeshEvent{
				EventId:   fmt.Sprintf("evt-%d", time.Now().UnixNano()),
				Topic:     req.GetTopic(),
				Payload:   payload,
				Timestamp: time.Now().Unix(),
			}

			if err := stream.Send(event); err != nil {
				return err
			}

			telemetry.RecordMeshBroadcast(ctx, "events")
			telemetry.RecordMeshMessageThroughput(ctx, len(payload), req.GetTopic())
		}
	}
}
