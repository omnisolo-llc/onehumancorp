package mesh

import (
	"context"
	"encoding/json"
	"time"

	pb "github.com/onehumancorp/mono/srcs/server/api/proto"
	"github.com/onehumancorp/mono/srcs/server/orchestration/mesh"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type CoordinationServer struct {
	pb.UnimplementedCoordinationServiceServer
	teammateMesh mesh.TeammateMesh
}

func NewCoordinationServer(tm mesh.TeammateMesh) *CoordinationServer {
	return &CoordinationServer{
		teammateMesh: tm,
	}
}

func (s *CoordinationServer) AcquireLock(ctx context.Context, req *pb.LockRequest) (*pb.LockResponse, error) {
	if req.AgentId == "" {
		return nil, status.Error(codes.InvalidArgument, "agent_id is required")
	}
	if req.TargetResource == "" {
		return nil, status.Error(codes.InvalidArgument, "target_resource is required")
	}
	if req.TtlSeconds <= 0 {
		return nil, status.Error(codes.InvalidArgument, "ttl_seconds must be positive")
	}

	ttl := time.Duration(req.TtlSeconds) * time.Second
	acquired, err := s.teammateMesh.AcquireLock(ctx, req.TargetResource, ttl)
	if err != nil {
		return &pb.LockResponse{
			Acquired:     false,
			ErrorMessage: err.Error(),
		}, nil
	}

	return &pb.LockResponse{
		Acquired: acquired,
	}, nil
}

func (s *CoordinationServer) ReleaseLock(ctx context.Context, req *pb.ReleaseRequest) (*pb.ReleaseResponse, error) {
	if req.AgentId == "" {
		return nil, status.Error(codes.InvalidArgument, "agent_id is required")
	}
	if req.TargetResource == "" {
		return nil, status.Error(codes.InvalidArgument, "target_resource is required")
	}

	err := s.teammateMesh.ReleaseLock(ctx, req.TargetResource)
	if err != nil {
		return &pb.ReleaseResponse{Success: false}, err
	}

	return &pb.ReleaseResponse{Success: true}, nil
}

func (s *CoordinationServer) StreamAgentState(req *pb.StateStreamRequest, stream pb.CoordinationService_StreamAgentStateServer) error {
	// Create a channel to receive messages from the subscription
	msgChan := make(chan []byte, 100)

	sub, err := s.teammateMesh.Subscribe(stream.Context(), "ohc.mesh.agent.status", func(msg []byte) {
		select {
		case msgChan <- msg:
		case <-stream.Context().Done():
		default:
			// Drop message if channel is full
		}
	})
	if err != nil {
		return status.Errorf(codes.Internal, "failed to subscribe: %v", err)
	}
	defer sub.Close()

	for {
		select {
		case <-stream.Context().Done():
			return nil
		case msg := <-msgChan:
			var update pb.StateUpdate
			if err := json.Unmarshal(msg, &update); err == nil {
				// Only stream if it matches domain filter or filter is empty
				if req.DomainFilter == "" /* || some filter logic */ {
					if err := stream.Send(&update); err != nil {
						return err
					}
				}
			}
		}
	}
}
