package orchestration

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// CachedMinimaxClient wraps a MinimaxClient and provides L1 (Redis) and L2 (SQLite/Postgres) caching
type CachedMinimaxClient struct {
	MinimaxClient
	db db.Provider
	redis rueidis.Client
}

// NewCachedMinimaxClient returns a new CachedMinimaxClient
func NewCachedMinimaxClient(client MinimaxClient, provider db.Provider, redis rueidis.Client) *CachedMinimaxClient {
	return &CachedMinimaxClient{
		MinimaxClient: client,
		db:            provider,
		redis:         redis,
	}
}

// GenerateEmbedding checks the cache before calling the underlying client
func (c *CachedMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	hashBytes := sha256.Sum256([]byte(text))
	contentHash := hex.EncodeToString(hashBytes[:])

	// 1. Check L1 Cache (Redis)
	if c.redis != nil {
		cmd := c.redis.B().Get().Key("embedding:" + contentHash).Build()
		res := c.redis.Do(ctx, cmd)
		if err := res.Error(); err == nil {
			val, _ := res.ToString()
			var embedding []float32
			if err := json.Unmarshal([]byte(val), &embedding); err == nil {
				return embedding, nil
			}
		} else if !rueidis.IsRedisNil(err) {
			slog.Warn("CachedMinimaxClient: Redis get failed", "error", err)
		}
	}

	// 2. Check L2 Cache (DB)
	if c.db != nil {
		var cachedEmbeddingStr string
		err := c.db.QueryRow(ctx, "SELECT embedding FROM embedding_cache WHERE content_hash = $1", contentHash).Scan(&cachedEmbeddingStr)
		if err == nil && cachedEmbeddingStr != "" {
			var embedding []float32
			if err := json.Unmarshal([]byte(cachedEmbeddingStr), &embedding); err == nil {
				// Populate L1 cache
				if c.redis != nil {
					cmd := c.redis.B().Set().Key("embedding:" + contentHash).Value(cachedEmbeddingStr).Nx().Px(24 * time.Hour).Build()
					c.redis.Do(ctx, cmd)
				}
				return embedding, nil
			}
		}
	}

	// 3. Fallback to API
	embedding, err := c.MinimaxClient.GenerateEmbedding(ctx, text)
	if err != nil {
		return nil, err
	}

	// 4. Save to Caches
	embeddingBytes, err := json.Marshal(embedding)
	if err == nil {
		embeddingStr := string(embeddingBytes)

		// L1 Cache
		if c.redis != nil {
			cmd := c.redis.B().Set().Key("embedding:" + contentHash).Value(embeddingStr).Nx().Px(24 * time.Hour).Build()
			c.redis.Do(ctx, cmd)
		}

		// L2 Cache
		if c.db != nil {
			query := "INSERT INTO embedding_cache (content_hash, embedding) VALUES ($1, $2) ON CONFLICT (content_hash) DO NOTHING"
			if c.db.IsSQLite() {
				query = "INSERT INTO embedding_cache (content_hash, embedding) VALUES (?, ?) ON CONFLICT (content_hash) DO NOTHING"
			}
			_, err = c.db.Exec(ctx, query, contentHash, embeddingStr)
			if err != nil {
				slog.Warn("CachedMinimaxClient: DB insert failed", "error", err)
			}
		}
	}

	return embedding, nil
}
