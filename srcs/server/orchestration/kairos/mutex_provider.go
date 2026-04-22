package kairos
import (
    "context"
    "time"
)

type Mutex interface {
    Lock(ctx context.Context, ttl time.Duration) error
    Unlock(ctx context.Context) error
}

type MutexProvider interface {
    NewMutex(key string) Mutex
}
