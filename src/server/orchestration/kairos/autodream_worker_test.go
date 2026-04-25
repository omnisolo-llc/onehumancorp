package kairos

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/telemetry"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
)

type MockEmbeddingClient struct{}

func (m *MockEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func setupTestDBAutoDream(t *testing.T) db.Provider {
	conn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	p := db.NewSqliteProvider(conn)
	ctx := context.Background()

	_, err = p.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id VARCHAR PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			processed_at TIMESTAMP WITH TIME ZONE
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return p
}

func TestAutoDreamConsolidator(t *testing.T) {
	// Fulfill telemetry requirement
	telemetry.InitTelemetry()

	p := setupTestDBAutoDream(t)
	defer p.Close()

	ctx := context.Background()
	_, err := p.Exec(ctx, "INSERT INTO autodream_memories (id, content) VALUES ('mem-1', 'test content 1')")
	assert.NoError(t, err)
	_, err = p.Exec(ctx, "INSERT INTO autodream_memories (id, content) VALUES ('mem-2', 'test content 2')")
	assert.NoError(t, err)

	mr, err := miniredis.Run()
	assert.NoError(t, err)
	defer mr.Close()

	redisClient := redis.NewClient(&redis.Options{Addr: mr.Addr()})

	consolidator := NewAutoDreamConsolidator(p, redisClient, &MockEmbeddingClient{})

	err = consolidator.ProcessBacklog(ctx)
	assert.NoError(t, err)

	var embedding string
	var processedAt time.Time
	err = p.QueryRow(ctx, "SELECT embedding, processed_at FROM autodream_memories WHERE id = 'mem-1'").Scan(&embedding, &processedAt)
	assert.NoError(t, err)
	assert.Equal(t, "[0.100000,0.200000,0.300000]", embedding)
	assert.False(t, processedAt.IsZero())
}
