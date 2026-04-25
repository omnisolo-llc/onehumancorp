package mesh

import (
    "context"
    "sync"
)

type localBrokerSubscription struct {
    broker  *LocalMeshBroker
    channel string
    ch      chan []byte
    handler func(msg []byte)
    ctx     context.Context
    cancel  context.CancelFunc
}

func (s *localBrokerSubscription) Close() error {
    s.cancel()
    s.broker.unsubscribe(s.channel, s)
    return nil
}

type LocalMeshBroker struct {
    mu               sync.RWMutex
    subscribers      map[string]chan []byte
    multiSubscribers map[string]map[*localBrokerSubscription]struct{}
}

func NewLocalMeshBroker() *LocalMeshBroker {
    return &LocalMeshBroker{
        subscribers:      make(map[string]chan []byte),
        multiSubscribers: make(map[string]map[*localBrokerSubscription]struct{}),
    }
}

func (b *LocalMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
    b.mu.RLock()
    defer b.mu.RUnlock()

    if ch, ok := b.subscribers[channel]; ok {
        select {
        case ch <- payload:
        case <-ctx.Done():
            return ctx.Err()
        default:
        }
    }

    if subs, ok := b.multiSubscribers[channel]; ok {
        for sub := range subs {
            select {
            case sub.ch <- payload:
            case <-ctx.Done():
                return ctx.Err()
            default:
            }
        }
    }

    return nil
}

func (b *LocalMeshBroker) Subscribe(ctx context.Context, channel string, handler func(msg []byte)) (Subscription, error) {
    subCtx, cancel := context.WithCancel(ctx)
    sub := &localBrokerSubscription{
        broker:  b,
        channel: channel,
        ch:      make(chan []byte, 100),
        handler: handler,
        ctx:     subCtx,
        cancel:  cancel,
    }

    b.mu.Lock()
    if b.multiSubscribers == nil {
        b.multiSubscribers = make(map[string]map[*localBrokerSubscription]struct{})
    }
    if b.multiSubscribers[channel] == nil {
        b.multiSubscribers[channel] = make(map[*localBrokerSubscription]struct{})
    }
    b.multiSubscribers[channel][sub] = struct{}{}
    b.mu.Unlock()

    go func() {
        for {
            select {
            case msg := <-sub.ch:
                sub.handler(msg)
            case <-subCtx.Done():
                b.unsubscribe(channel, sub)
                return
            }
        }
    }()

    return sub, nil
}

func (b *LocalMeshBroker) unsubscribe(channel string, sub *localBrokerSubscription) {
    b.mu.Lock()
    defer b.mu.Unlock()
    if subs, ok := b.multiSubscribers[channel]; ok {
        delete(subs, sub)
        if len(subs) == 0 {
            delete(b.multiSubscribers, channel)
        }
    }
}
