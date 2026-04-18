package services

import (
	"context"
	"database/sql"
	"time"
	"fmt"

	"github.com/redis/rueidis"
)

type DistributedLockService struct {
	isRedis     bool
	redisClient rueidis.Client
	db          *sql.DB
}

func NewDistributedLockService(redisClient rueidis.Client, db *sql.DB) *DistributedLockService {
	return &DistributedLockService{
		isRedis:     redisClient != nil,
		redisClient: redisClient,
		db:          db,
	}
}

func (s *DistributedLockService) AcquireLock(ctx context.Context, key string, expiration time.Duration, token string) (bool, error) {
	if s.isRedis {
		cmd := s.redisClient.B().Set().Key(key).Value(token).Nx().Ex(expiration).Build()
		err := s.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return false, nil
			}
			return false, fmt.Errorf("redis set error: %w", err)
		}
		return true, nil
	}

	// Standalone / SQLite implementation fallback via DB
	// Just use a dummy implementation for tests for now.
	// We're focusing on redis rueidis logic.
	return true, nil
}

func (s *DistributedLockService) ReleaseLock(ctx context.Context, key string, token string) error {
	if s.isRedis {
		script := rueidis.NewLuaScript(`
if redis.call("get",KEYS[1]) == ARGV[1] then
    return redis.call("del",KEYS[1])
else
    return 0
end`)
		err := script.Exec(ctx, s.redisClient, []string{key}, []string{token}).Error()
		return err
	}
	return nil
}
