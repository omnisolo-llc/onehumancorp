package queue

import (
	"context"
	"sync"
)

type InMemJobQueue struct {
	topics sync.Map
}

func NewInMemJobQueue() *InMemJobQueue {
	return &InMemJobQueue{}
}

func (q *InMemJobQueue) getChan(topic string) chan []byte {
	if v, ok := q.topics.Load(topic); ok {
		return v.(chan []byte)
	}
	v, _ := q.topics.LoadOrStore(topic, make(chan []byte, 10000))
	return v.(chan []byte)
}

func (q *InMemJobQueue) Push(ctx context.Context, topic string, payload []byte) error {
	select {
	case q.getChan(topic) <- payload:
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}

func (q *InMemJobQueue) Pop(ctx context.Context, topic string) ([]byte, error) {
	select {
	case payload := <-q.getChan(topic):
		return payload, nil
	case <-ctx.Done():
		return nil, ctx.Err()
	}
}
