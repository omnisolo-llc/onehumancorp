package interop

import "context"

type Message struct {
	Topic   string
	Payload []byte
}

type Bus interface {
	Publish(ctx context.Context, msg Message) error
	Subscribe(ctx context.Context, topic string, handler func(Message)) (func(), error)
}

type DistributedLock interface {
	AcquireLock(ctx context.Context, resource string, owner string, ttlSeconds int) (bool, error)
	ReleaseLock(ctx context.Context, resource string, owner string) error
}
