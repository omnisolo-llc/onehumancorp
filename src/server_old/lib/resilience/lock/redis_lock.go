package lock

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/redis/rueidis"
)

// RedisLockProvider implements a distributed lock using Redis.
type RedisLockProvider struct {
	client rueidis.Client
}

// NewRedisLockProvider creates a new RedisLockProvider.
func NewRedisLockProvider(client rueidis.Client) *RedisLockProvider {
	return &RedisLockProvider{
		client: client,
	}
}

var unlockScript = rueidis.NewLuaScript(`
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`)

// TryLock attempts to acquire a lock using Redis.
func (p *RedisLockProvider) TryLock(ctx context.Context, key string, ttl time.Duration) (bool, func(context.Context) error, error) {
	token := uuid.New().String()

	cmd := p.client.B().Set().Key(key).Value(token).Nx().Px(ttl).Build()
	err := p.client.Do(ctx, cmd).Error()
	if err != nil {
		if rueidis.IsRedisNil(err) {
			return false, nil, nil
		}
		return false, nil, err
	}

	unlock := func(unlockCtx context.Context) error {
		return unlockScript.Exec(unlockCtx, p.client, []string{key}, []string{token}).Error()
	}

	return true, unlock, nil
}
