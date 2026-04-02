package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/proto"
)

// Task represents a unit of work from the Shared Task List.
type Task struct {
	ID              string
	MissionID       string
	Title           string
	Status          string // PENDING, IN_PROGRESS, COMPLETED, FAILED
	AssignedAgentID string
	LockedUntil     time.Time
	Payload         json.RawMessage
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

// ClaimTask attempts to assign a PENDING task to the requesting agent.
// It uses a distributed lock (via Redis if available, or DB lock in Standalone mode)
// to prevent "split-brain" claiming.
func (h *Hub) ClaimTask(ctx context.Context, taskID, agentID string) (*Task, error) {
	sipDB := h.SIPDB()
	if sipDB == nil {
		return nil, errors.New("SIPDB not configured")
	}

	// Wait, we need to implement this in SIPDB to execute the claim query correctly.
	// But let's delegate to sipDB.ClaimTask
	task, err := sipDB.ClaimTask(ctx, taskID, agentID)
	if err != nil {
		return nil, err
	}

	return task, nil
}

// ClaimTask implementation for HubServiceServer.
func (s *HubServiceServer) ClaimTask(ctx context.Context, req *pb.ClaimTaskRequest) (*pb.ClaimTaskResponse, error) {
	task, err := s.hub.ClaimTask(ctx, req.GetTaskId(), req.GetAgentId())
	if err != nil {
		return nil, status.Errorf(codes.Internal, "claim task failed: %v", err)
	}

	payloadStr := string(task.Payload)

	pbTask := &pb.SwarmTask{
		Id:               task.ID,
		MissionId:        task.MissionID,
		Title:            task.Title,
		Status:           task.Status,
		AssignedAgentId:  task.AssignedAgentID,
		LockedUntilUnix:  task.LockedUntil.Unix(),
		Payload:          payloadStr,
		CreatedAtUnix:    task.CreatedAt.Unix(),
		UpdatedAtUnix:    task.UpdatedAt.Unix(),
	}

	return &pb.ClaimTaskResponse{
		Success: true,
		Task:    pbTask,
	}, nil
}
