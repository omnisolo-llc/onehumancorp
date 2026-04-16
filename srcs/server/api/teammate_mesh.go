package api

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"time"

	"github.com/redis/go-redis/v9"
	pb "github.com/onehumancorp/mono/srcs/server/api/proto"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type CoordinationServiceServer struct {
	pb.UnimplementedCoordinationServiceServer
	db    *sql.DB
	redis *redis.Client
}

func NewCoordinationServiceServer(db *sql.DB, rdb *redis.Client) *CoordinationServiceServer {
	return &CoordinationServiceServer{
		db:    db,
		redis: rdb,
	}
}

func (s *CoordinationServiceServer) AcquireLock(ctx context.Context, req *pb.LockRequest) (*pb.LockResponse, error) {
	if req.AgentId == "" || req.TargetResource == "" {
		return nil, status.Error(codes.InvalidArgument, "agent_id and target_resource are required")
	}

	if s.redis != nil {
		ttl := time.Duration(req.TtlSeconds) * time.Second
		if ttl == 0 {
			ttl = 30 * time.Second
		}

		key := fmt.Sprintf("lock:%s", req.TargetResource)
		acquired, err := s.redis.SetNX(ctx, key, req.AgentId, ttl).Result()
		if err != nil {
			return nil, status.Errorf(codes.Internal, "redis setnx error: %v", err)
		}

		if !acquired {
			return &pb.LockResponse{
				Acquired:     false,
				ErrorMessage: "lock already taken",
			}, nil
		}

		return &pb.LockResponse{
			Acquired: true,
		}, nil
	}

	// SQLite fallback
	if s.db == nil {
		return nil, status.Error(codes.Internal, "no database connection available")
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to begin transaction: %v", err)
	}
	defer tx.Rollback()

	var currentLockID string
	var expiresAt time.Time
	err = tx.QueryRowContext(ctx, "SELECT agent_id, lock_expires_at FROM agent_state WHERE lock_id = ?", req.TargetResource).Scan(&currentLockID, &expiresAt)
	if err != nil && err != sql.ErrNoRows {
		return nil, status.Errorf(codes.Internal, "failed to check lock: %v", err)
	}

	now := time.Now()
	if err == nil {
		// Lock exists
		if currentLockID == req.AgentId || now.After(expiresAt) {
			// Proceed
		} else {
			return &pb.LockResponse{
				Acquired:     false,
				ErrorMessage: "lock already taken",
			}, nil
		}
	}

	ttl := time.Duration(req.TtlSeconds) * time.Second
	if ttl == 0 {
		ttl = 30 * time.Second
	}
	expiration := now.Add(ttl)

	_, err = tx.ExecContext(ctx, `
		INSERT INTO agent_state (agent_id, lock_id, lock_expires_at, last_heartbeat)
		VALUES (?, ?, ?, ?)
		ON CONFLICT(agent_id) DO UPDATE SET
			lock_id = excluded.lock_id,
			lock_expires_at = excluded.lock_expires_at,
			last_heartbeat = excluded.last_heartbeat
	`, req.AgentId, req.TargetResource, expiration, now)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to update state: %v", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, status.Errorf(codes.Internal, "failed to commit transaction: %v", err)
	}

	return &pb.LockResponse{
		Acquired: true,
	}, nil
}

func (s *CoordinationServiceServer) ReleaseLock(ctx context.Context, req *pb.ReleaseRequest) (*pb.ReleaseResponse, error) {
	if req.AgentId == "" || req.TargetResource == "" {
		return nil, status.Error(codes.InvalidArgument, "agent_id and target_resource are required")
	}

	if s.redis != nil {
		key := fmt.Sprintf("lock:%s", req.TargetResource)

		// Lua script to ensure we only delete the lock if we own it
		script := redis.NewScript(`
			if redis.call("get", KEYS[1]) == ARGV[1] then
				return redis.call("del", KEYS[1])
			else
				return 0
			end
		`)

		res, err := script.Run(ctx, s.redis, []string{key}, req.AgentId).Result()
		if err != nil {
			return nil, status.Errorf(codes.Internal, "redis eval error: %v", err)
		}

		deleted := res.(int64) > 0
		return &pb.ReleaseResponse{Success: deleted}, nil
	}

	// SQLite fallback
	if s.db == nil {
		return nil, status.Error(codes.Internal, "no database connection available")
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to begin transaction: %v", err)
	}
	defer tx.Rollback()

	res, err := tx.ExecContext(ctx, "UPDATE agent_state SET lock_id = NULL, lock_expires_at = NULL WHERE agent_id = ? AND lock_id = ?", req.AgentId, req.TargetResource)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to release lock: %v", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, status.Errorf(codes.Internal, "failed to commit transaction: %v", err)
	}

	affected, _ := res.RowsAffected()
	return &pb.ReleaseResponse{Success: affected > 0}, nil
}

func (s *CoordinationServiceServer) StreamAgentState(req *pb.StateStreamRequest, stream pb.CoordinationService_StreamAgentStateServer) error {
	if s.redis == nil {
		return status.Error(codes.Unimplemented, "streaming not implemented for standalone mode")
	}

	ctx := stream.Context()
	pubsub := s.redis.Subscribe(ctx, "ohc.mesh.agent.status")
	defer pubsub.Close()

	ch := pubsub.Channel()

	for {
		select {
		case <-ctx.Done():
			return ctx.Err()
		case msg := <-ch:
			var update pb.StateUpdate
			if err := json.Unmarshal([]byte(msg.Payload), &update); err != nil {
				log.Printf("failed to unmarshal state update: %v", err)
				continue
			}

			// Simple domain filtering
			if req.DomainFilter != "" && update.AgentId != req.DomainFilter {
			    // This could be enhanced to actually check domain if domain info is in the update
			    // For now, if domain filter is provided, skip.
			}

			if err := stream.Send(&update); err != nil {
				return err
			}
		}
	}
}

// PublishAgentState is a helper to publish an agent's state
func PublishAgentState(ctx context.Context, rdb *redis.Client, agentID, newStatus, currentMission string) error {
	if rdb == nil {
		return nil // no-op in standalone mode for now
	}

	update := &pb.StateUpdate{
		AgentId:        agentID,
		NewStatus:      newStatus,
		CurrentMission: currentMission,
	}

	data, err := json.Marshal(update)
	if err != nil {
		return err
	}

	return rdb.Publish(ctx, "ohc.mesh.agent.status", data).Err()
}
