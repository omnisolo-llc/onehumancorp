package orchestration

import (
	"context"
	"encoding/json"
	"time"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// AgentCapabilities is a stub for the pb package
type AgentCapabilities struct {
	AgentId string
	Capabilities []string
}

type EmptyResponse struct{}

type MeshEvent struct {
	EventType string
	AgentId string
	Payload string
	TimestampUnix int64
}

type DiscoverAgentsRequest struct {}
type DiscoverAgentsResponse struct {}

type EventStreamRequest struct {
	AgentId string
	Channels []string
}

type TeammateMeshService_StreamMeshEventsServer interface {
	Context() context.Context
	Send(*MeshEvent) error
}

type TeammateMeshServer struct {
	meshTransport MeshHub
}

func NewTeammateMeshServer(meshTransport MeshHub) *TeammateMeshServer {
	return &TeammateMeshServer{
		meshTransport: meshTransport,
	}
}

func (s *TeammateMeshServer) AdvertiseCapabilities(ctx context.Context, req *AgentCapabilities) (*EmptyResponse, error) {
	if req.AgentId == "" {
		return nil, status.Error(codes.InvalidArgument, "agent_id is required")
	}

	payloadBytes, _ := json.Marshal(req)

	event := &MeshEvent{
		EventType:     "CAPABILITIES_ADVERTISED",
		AgentId:       req.AgentId,
		Payload:       string(payloadBytes),
		TimestampUnix: time.Now().Unix(),
	}

	eventBytes, _ := json.Marshal(event)

	err := s.meshTransport.Publish(ctx, "mesh:presence", eventBytes)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to publish mesh event: %v", err)
	}

	return &EmptyResponse{}, nil
}

func (s *TeammateMeshServer) DiscoverAgents(ctx context.Context, req *DiscoverAgentsRequest) (*DiscoverAgentsResponse, error) {
	return &DiscoverAgentsResponse{}, nil
}

func (s *TeammateMeshServer) StreamMeshEvents(req *EventStreamRequest, stream TeammateMeshService_StreamMeshEventsServer) error {
	if req.AgentId == "" {
		return status.Error(codes.InvalidArgument, "agent_id is required")
	}

	ctx := stream.Context()
	eventCh := make(chan *MeshEvent, 100)

	channels := req.Channels
	if len(channels) == 0 {
		channels = []string{"mesh:tasks", "mesh:presence"}
	}

	for _, channel := range channels {
		err := s.meshTransport.Subscribe(ctx, channel, func(data []byte) {
			var event MeshEvent
			if err := json.Unmarshal(data, &event); err == nil {
				select {
				case eventCh <- &event:
				default:
				}
			}
		})
		if err != nil {
			return status.Errorf(codes.Internal, "failed to subscribe to channel %s: %v", channel, err)
		}
	}

	for {
		select {
		case <-ctx.Done():
			return nil
		case event := <-eventCh:
			if err := stream.Send(event); err != nil {
				return err
			}
		}
	}
}
