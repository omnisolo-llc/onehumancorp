package benchmarks

import (
	"context"
	"fmt"
	"sync"
	"testing"
	"time"
)

type Task struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	TaskID  string `json:"task_id"`
}

const numShards = 16

type MemoryMeshTransport struct {
	broadcast       []chan Task
	mu              []sync.RWMutex
	subs            []map[chan Task]struct{}
}

func NewMemoryMeshTransport() *MemoryMeshTransport {
	lm := &MemoryMeshTransport{
		broadcast:       make([]chan Task, numShards),
		mu:              make([]sync.RWMutex, numShards),
		subs:            make([]map[chan Task]struct{}, numShards),
	}

	for i := 0; i < numShards; i++ {
		lm.broadcast[i] = make(chan Task, 1000000)
		lm.subs[i] = make(map[chan Task]struct{})

		for j := 0; j < 4; j++ {
			go lm.run(i)
		}
	}
	return lm
}

func (lm *MemoryMeshTransport) getShard(key string) int {
	var hash uint32
	for i := 0; i < len(key); i++ {
		hash = hash*31 + uint32(key[i])
	}
	return int(hash % numShards)
}

func (lm *MemoryMeshTransport) BroadcastTask(ctx context.Context, task Task) error {
	shardIdx := lm.getShard(task.TaskID)
	// Non-blocking fast path
    select {
    case lm.broadcast[shardIdx] <- task:
        return nil
    default:
        // simulate backoff
        time.Sleep(time.Microsecond)
        lm.broadcast[shardIdx] <- task
        return nil
    }
}

func (lm *MemoryMeshTransport) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	ch := make(chan Task, 100000)

	for i := 0; i < numShards; i++ {
		lm.mu[i].Lock()
		lm.subs[i][ch] = struct{}{}
		lm.mu[i].Unlock()
	}

	return ch, nil
}

func (lm *MemoryMeshTransport) run(shardIdx int) {
	for msg := range lm.broadcast[shardIdx] {
		lm.mu[shardIdx].RLock()
		for ch := range lm.subs[shardIdx] {
			select {
			case ch <- msg:
			default:
			}
		}
		lm.mu[shardIdx].RUnlock()
	}
}

type LegacyMeshTransport struct {
	broadcast chan Task
	mu        sync.RWMutex
	subs      map[chan Task]struct{}
}

func NewLegacyMeshTransport() *LegacyMeshTransport {
	lm := &LegacyMeshTransport{
		broadcast: make(chan Task, 1000000),
		subs:      make(map[chan Task]struct{}),
	}
	go lm.run()
	return lm
}

func (lm *LegacyMeshTransport) BroadcastTask(ctx context.Context, task Task) error {
    select {
    case lm.broadcast <- task:
        return nil
    default:
        time.Sleep(time.Microsecond)
        lm.broadcast <- task
        return nil
    }
}

func (lm *LegacyMeshTransport) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	ch := make(chan Task, 100000)
    lm.mu.Lock()
    lm.subs[ch] = struct{}{}
    lm.mu.Unlock()
	return ch, nil
}

func (lm *LegacyMeshTransport) run() {
	for msg := range lm.broadcast {
		lm.mu.RLock()
		for ch := range lm.subs {
			select {
			case ch <- msg:
			default:
			}
		}
		lm.mu.RUnlock()
	}
}

func BenchmarkLegacyTeammateMesh_ParallelBroadcastTask(b *testing.B) {
	mesh := NewLegacyMeshTransport()
    ctx := context.Background()

    for i := 0; i < 100; i++ {
        mesh.SubscribeTasks(ctx)
    }

	b.ResetTimer()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			task := Task{
				AgentID: "system",
				TaskID:  fmt.Sprintf("task-%d", i),
			}
			_ = mesh.BroadcastTask(ctx, task)
			i++
		}
	})
}

func BenchmarkLocalTeammateMesh_ParallelBroadcastTask(b *testing.B) {
	mesh := NewMemoryMeshTransport()
    ctx := context.Background()

    for i := 0; i < 100; i++ {
        mesh.SubscribeTasks(ctx)
    }

	b.ResetTimer()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			task := Task{
				AgentID: "system",
				TaskID:  fmt.Sprintf("task-%d", i),
			}
			_ = mesh.BroadcastTask(ctx, task)
			i++
		}
	})
}
