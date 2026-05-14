package orchestration

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/go-redis/redis/v8"
	"github.com/pgvector/pgvector-go"
	"gorm.io/gorm"
)

// AutoDream Pipeline using pgvector for memory consolidation
type AutoDream struct {
	db *gorm.DB
}

func NewAutoDream(db *gorm.DB) *AutoDream {
	return &AutoDream{
		db: db,
	}
}

// Consolidate consolidates memory into pgvector
func (a *AutoDream) Consolidate(memory string, embedding []float32) error {
	if a.db == nil {
		return nil // Mock implementation for testing
	}

	vec := pgvector.NewVector(embedding)

	// In a real implementation, this inserts into pgvector consolidated_memory table
	return a.db.Exec("INSERT INTO consolidated_memory (embedding, content, created_at) VALUES (?, ?, CURRENT_TIMESTAMP)", vec, memory).Error
}

func (a *AutoDream) QueryMemory(queryEmbedding []float32, limit int) ([]string, error) {
	if a.db == nil {
		return []string{"mock_memory"}, nil // Mock implementation for testing
	}

	vec := pgvector.NewVector(queryEmbedding)

	var memories []string
	// Using pgvector's L2 distance (<->) for similarity search
	err := a.db.Raw("SELECT content FROM consolidated_memory ORDER BY embedding <-> ? LIMIT ?", vec, limit).Scan(&memories).Error
	return memories, err
}

func (a *AutoDream) CleanupOldMemories(days int) error {
	if a.db == nil {
		return nil
	}
	// Fixed the INTERVAL query parameterization issue
	return a.db.Exec("DELETE FROM consolidated_memory WHERE created_at < CURRENT_TIMESTAMP - (? * INTERVAL '1 day')", days).Error
}

// Teammate Mesh using Redis Pub/Sub with in-memory fallback
type TeammateMesh struct {
	redisClient *redis.Client
	mu          sync.Mutex
	subscribers map[string][]chan string // channel -> slice of subscriber channels
	ctx         context.Context
}

func NewTeammateMesh(redisURL string) *TeammateMesh {
	tm := &TeammateMesh{
		subscribers: make(map[string][]chan string),
		ctx:         context.Background(),
	}
	if redisURL != "" {
		tm.redisClient = redis.NewClient(&redis.Options{
			Addr: redisURL,
		})
	}
	return tm
}

func (t *TeammateMesh) Broadcast(channel, msg string) error {
	if t.redisClient != nil {
		return t.redisClient.Publish(t.ctx, channel, msg).Err()
	}

	// Fallback logic - push to all active subscribers for the channel
	t.mu.Lock()
	defer t.mu.Unlock()

	if subs, exists := t.subscribers[channel]; exists {
		for _, subCh := range subs {
			select {
			case subCh <- msg:
				// Successfully pushed to subscriber
			default:
				// If subscriber is too slow and channel is full, drop the message
				// rather than deadlocking the entire broadcast operation
			}
		}
	}

	return nil
}

func (t *TeammateMesh) Subscribe(ctx context.Context, channel string) (<-chan string, error) {
	msgChan := make(chan string, 100)

	if t.redisClient != nil {
		pubsub := t.redisClient.Subscribe(ctx, channel)
		go func() {
			defer pubsub.Close()
			for {
				msg, err := pubsub.ReceiveMessage(ctx)
				if err != nil {
					close(msgChan)
					return
				}
				select {
				case <-ctx.Done():
					close(msgChan)
					return
				case msgChan <- msg.Payload:
				}
			}
		}()
		return msgChan, nil
	}

	// Fallback logic - register the channel
	t.mu.Lock()
	defer t.mu.Unlock()

	t.subscribers[channel] = append(t.subscribers[channel], msgChan)

	// Handle cleanup on context cancellation to prevent goroutine and memory leaks
	go func() {
		<-ctx.Done()
		t.mu.Lock()
		defer t.mu.Unlock()

		if subs, exists := t.subscribers[channel]; exists {
			for i, ch := range subs {
				if ch == msgChan {
					// Remove the channel from subscribers
					t.subscribers[channel] = append(subs[:i], subs[i+1:]...)
					break
				}
			}
		}
		close(msgChan)
	}()

	return msgChan, nil
}
