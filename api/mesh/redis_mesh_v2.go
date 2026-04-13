package mesh

import (
	"context"
	"errors"
	"fmt"
	"strconv"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/redis/rueidis"
)

type RedisMeshV2 struct {
	client rueidis.Client
}

func NewRedisMeshV2(client rueidis.Client) *RedisMeshV2 {
	return &RedisMeshV2{
		client: client,
	}
}

type rueidisSubscription struct {
	cancel context.CancelFunc
}

func (s *rueidisSubscription) Unsubscribe() error {
	s.cancel()
	return nil
}

func (rm *RedisMeshV2) Publish(ctx context.Context, topic string, payload []byte) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	broadcastCount.Add(ctx, 1)

	cmd := rm.client.B().Publish().Channel(topic).Message(string(payload)).Build()
	return rm.client.Do(ctx, cmd).Error()
}

func (rm *RedisMeshV2) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	subscribeCount.Add(ctx, 1)

	subCtx, cancel := context.WithCancel(ctx)

	go func() {
		err := rm.client.Receive(subCtx, rm.client.B().Subscribe().Channel(topic).Build(), func(msg rueidis.PubSubMessage) {
			handler([]byte(msg.Message))
		})
		if err != nil && err != context.Canceled {
			// In a real application, handle error logging
		}
	}()

	return &rueidisSubscription{cancel: cancel}, nil
}

func (rm *RedisMeshV2) AcquireLock(ctx context.Context, key string, lockID string, ttl time.Duration) (bool, error) {
	cmd := rm.client.B().Set().Key(fmt.Sprintf("lock:%s", key)).Value(lockID).Nx().Px(ttl).Build()
	err := rm.client.Do(ctx, cmd).Error()
	if err != nil {
		if rueidis.IsRedisNil(err) {
			return false, nil // Could not acquire lock
		}
		return false, fmt.Errorf("redis setnx error: %w", err)
	}
	return true, nil
}

func (rm *RedisMeshV2) ReleaseLock(ctx context.Context, key string, lockID string) error {
	script := `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`
	cmd := rm.client.B().Eval().Script(script).Numkeys(1).Key(fmt.Sprintf("lock:%s", key)).Arg(lockID).Build()
	val, err := rm.client.Do(ctx, cmd).AsInt64()
	if err != nil {
		return fmt.Errorf("redis eval error: %w", err)
	}
	if val == 0 {
		return errors.New("lock is not owned or has expired")
	}
	return nil
}

func (rm *RedisMeshV2) RegisterPresence(ctx context.Context, agentID string, status string) error {
	now := time.Now().UnixMilli()

	err := rm.client.Do(ctx, rm.client.B().Zadd().Key("agents:presence:zset").ScoreMember().ScoreMember(float64(now), agentID).Build()).Error()
	if err != nil {
		return err
	}
	return rm.client.Do(ctx, rm.client.B().Hset().Key("agents:presence:status").FieldValue().FieldValue(agentID, status).Build()).Error()
}

func (rm *RedisMeshV2) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	now := time.Now().UnixMilli()
	cutoff := float64(now - 30000) // 30 seconds ago

    // Find old agents to remove from hash
    oldMembers, err := rm.client.Do(ctx, rm.client.B().Zrangebyscore().Key("agents:presence:zset").Min("-inf").Max(strconv.FormatFloat(cutoff, 'f', -1, 64)).Build()).AsStrSlice()
    if err == nil && len(oldMembers) > 0 {
        _ = rm.client.Do(ctx, rm.client.B().Hdel().Key("agents:presence:status").Field(oldMembers...).Build()).Error()
    }

	_ = rm.client.Do(ctx, rm.client.B().Zremrangebyscore().Key("agents:presence:zset").Min("-inf").Max(strconv.FormatFloat(cutoff, 'f', -1, 64)).Build()).Error()

	members, err := rm.client.Do(ctx, rm.client.B().Zrange().Key("agents:presence:zset").Min("0").Max("-1").Withscores().Build()).AsStrSlice()
	if err != nil {
		return nil, err
	}

    if len(members) == 0 {
        return []AgentPresence{}, nil
    }

    var agentIDs []string
    for i := 0; i < len(members); i += 2 {
        agentIDs = append(agentIDs, members[i])
    }

    // hmget
    cmd := rm.client.B().Hmget().Key("agents:presence:status").Field(agentIDs...).Build()

    statuses, err := rm.client.Do(ctx, cmd).AsStrSlice()
    if err != nil {
        return nil, err
    }

	var agents []AgentPresence
	for i := 0; i < len(members); i += 2 {
		agentID := members[i]
		scoreStr := members[i+1]

		scoreFloat, _ := strconv.ParseFloat(scoreStr, 64)

		status := "UNKNOWN"
        if len(statuses) > i/2 {
            status = statuses[i/2]
        }

		agents = append(agents, AgentPresence{
			AgentID:  agentID,
			Status:   status,
			LastSeen: time.UnixMilli(int64(scoreFloat)),
		})
	}
	return agents, nil
}
