package memory

import (
	"context"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
)

// MemoryService interface for agent episodic memory cache
type MemoryService interface {
	SaveEpisodicMemory(ctx context.Context, tenantID, sessionID, context string) error
	RetrieveRecentMemory(ctx context.Context, tenantID, sessionID string) (string, error)
}

// AgentMemoryService implementation
type AgentMemoryService struct {
	client *redis.Client
}

// NewAgentMemoryService creates a new memory service
func NewAgentMemoryService(client *redis.Client) *AgentMemoryService {
	return &AgentMemoryService{
		client: client,
	}
}

// key constructs the isolated tenant key
func key(tenantID, sessionID string) string {
	return fmt.Sprintf("ohc:mem:%s:%s", tenantID, sessionID)
}

// SaveEpisodicMemory saves memory to Redis cache strictly isolated by tenant ID
func (s *AgentMemoryService) SaveEpisodicMemory(ctx context.Context, tenantID, sessionID, memoryContext string) error {
	return s.client.Set(ctx, key(tenantID, sessionID), memoryContext, 7*24*time.Hour).Err()
}

// RetrieveRecentMemory gets recent memory isolated by tenant ID
func (s *AgentMemoryService) RetrieveRecentMemory(ctx context.Context, tenantID, sessionID string) (string, error) {
	val, err := s.client.Get(ctx, key(tenantID, sessionID)).Result()
	if err == redis.Nil {
		return "", nil // Return empty string for not found instead of error
	} else if err != nil {
		return "", err
	}
	return val, nil
}
