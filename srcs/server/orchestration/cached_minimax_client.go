package orchestration

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

type CachedMinimaxClient struct {
	baseClient MinimaxClient
	db         db.Provider
	redis      rueidis.Client
}

func NewCachedMinimaxClient(baseClient MinimaxClient, dbProvider db.Provider, redisClient rueidis.Client) *CachedMinimaxClient {
	return &CachedMinimaxClient{
		baseClient: baseClient,
		db:         dbProvider,
		redis:      redisClient,
	}
}

func (c *CachedMinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	return c.baseClient.Reason(ctx, prompt)
}

func (c *CachedMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	hashBytes := sha256.Sum256([]byte(text))
	contentHash := hex.EncodeToString(hashBytes[:])

	// 1. Check L1 Cache (Redis)
	if c.redis != nil {
		cmd := c.redis.B().Get().Key("embedding:" + contentHash).Build()
		val, err := c.redis.Do(ctx, cmd).ToString()
		if err == nil && val != "" {
			var embedding []float32
			if err := json.Unmarshal([]byte(val), &embedding); err == nil {
				return embedding, nil
			}
		}
	}

	// 2. Check L2 Cache (DB)
	var cachedEmbeddingStr string
	var err error
	if c.db != nil {
		err = c.db.QueryRow(ctx, "SELECT embedding FROM embedding_cache WHERE content_hash = $1", contentHash).Scan(&cachedEmbeddingStr)
		if err == nil && cachedEmbeddingStr != "" {
			var embedding []float32
			if err := json.Unmarshal([]byte(cachedEmbeddingStr), &embedding); err == nil {
				// Populate L1 cache before returning
				if c.redis != nil {
					cmd := c.redis.B().Set().Key("embedding:" + contentHash).Value(cachedEmbeddingStr).Ex(24 * time.Hour).Build()
					c.redis.Do(ctx, cmd)
				}
				return embedding, nil
			}
		}
	}

	// 3. Call Underlying
	embedding, err := c.baseClient.GenerateEmbedding(ctx, text)
	if err != nil {
		return nil, err
	}

	// 4. Save to Cache
	embeddingBytes, _ := json.Marshal(embedding)
	embeddingStr := string(embeddingBytes)

	if c.redis != nil {
		cmd := c.redis.B().Set().Key("embedding:" + contentHash).Value(embeddingStr).Ex(24 * time.Hour).Build()
		c.redis.Do(ctx, cmd)
	}

	if c.db != nil {
		cacheQuery := "INSERT INTO embedding_cache (content_hash, embedding) VALUES ($1, $2) ON CONFLICT (content_hash) DO NOTHING"
		if c.db.IsSQLite() {
			cacheQuery = "INSERT INTO embedding_cache (content_hash, embedding) VALUES (?, ?) ON CONFLICT (content_hash) DO NOTHING"
		}
		_, err := c.db.Exec(ctx, cacheQuery, contentHash, embeddingStr)
		if err != nil {
			slog.Warn("CachedMinimaxClient: failed to save embedding to db", "err", err)
		}
	}

	return embedding, nil
}
