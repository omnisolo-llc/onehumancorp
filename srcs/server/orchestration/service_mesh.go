package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"google.golang.org/protobuf/proto"
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

	if telemetry.BufferMetricFunc == nil {
		telemetry.RecordMeshBroadcast(ctx, "capabilities")
	} else {
		payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "capabilities"})
		_ = telemetry.BufferMetricFunc(ctx, "mesh_broadcast", string(payloadBytes))
	}

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

			if err := stream.Send(&caps); err != nil {
				return err
			}
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

			if telemetry.BufferMetricFunc == nil {
				telemetry.RecordMeshBroadcast(ctx, "events")
			} else {
				payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "events"})
				_ = telemetry.BufferMetricFunc(ctx, "mesh_broadcast", string(payloadBytes))
			}
		}
	}
}


// BroadcastMeshEvent broadcasts a message to the entire Teammate Mesh
func (s *HubServiceServer) BroadcastMeshEvent(ctx context.Context, req *pb.MeshEvent) (*pb.PublishMessageResponse, error) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(ctx, "BroadcastMeshEvent", time.Since(start)) }()

	if req.GetTopic() == "" {
		return nil, fmt.Errorf("topic is required")
	}

	cn := s.hub.CentrifugeNode()
	if cn == nil {
		return nil, fmt.Errorf("CentrifugeNode is nil")
	}

	if cn.meshTransport == nil {
		return nil, fmt.Errorf("meshTransport is not configured")
	}

	if err := cn.meshTransport.BroadcastMeshEvent(ctx, req.GetTopic(), req.GetPayload()); err != nil {
		slog.Error("Failed to broadcast mesh event", "error", err, "topic", req.GetTopic())
		return nil, fmt.Errorf("failed to broadcast event: %w", err)
	}

	if telemetry.BufferMetricFunc == nil {
		telemetry.RecordMeshBroadcast(ctx, "event_broadcast")
	} else {
		payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "event_broadcast"})
		_ = telemetry.BufferMetricFunc(ctx, "mesh_broadcast", string(payloadBytes))
	}

	return &pb.PublishMessageResponse{Success: true}, nil
}

// StreamTasks streams real-time tasks updates via SSE/WebSocket
func (s *HubServiceServer) StreamTasks(req *pb.EventStreamRequest, stream pb.HubService_StreamTasksServer) error {
	ctx := stream.Context()

	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(ctx, "StreamTasks", time.Since(start)) }()

	cn := s.hub.CentrifugeNode()
	if cn == nil {
		return fmt.Errorf("CentrifugeNode is nil")
	}

	if cn.meshTransport == nil {
		return fmt.Errorf("meshTransport is not configured")
	}

	// We utilize the same mesh transport, subscribing to a "tasks" specific topic
	eventsChan, err := cn.meshTransport.SubscribeMeshEvents(ctx, "tasks")
	if err != nil {
		return fmt.Errorf("failed to subscribe to task events: %w", err)
	}

	for {
		select {
		case <-ctx.Done():
			return nil
		case payload, ok := <-eventsChan:
			if !ok {
				return nil
			}

			// We wrap it into whatever TaskStreamResponse the proto demands.
			// Assuming it expects MeshEvent for simplicity or equivalent structure.
			// To be robust, let's just marshal it as a task stream event.
			var taskID string

			// Trying to extract task ID from payload
			var payloadMap map[string]interface{}
			if err := json.Unmarshal(payload, &payloadMap); err == nil {
				if t, ok := payloadMap["task_id"].(string); ok {
					taskID = t
				}
			}

			event := &pb.TaskStreamResponse{
				TaskId: taskID,
				Payload: payload,
			}

			if err := stream.Send(event); err != nil {
				return err
			}
		}
	}
}
