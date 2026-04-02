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

// CachedMinimaxClient wraps a MinimaxClient and caches embeddings
// in Redis (if available) and the DB embedding_cache table.
type CachedMinimaxClient struct {
	client MinimaxClient
	db     db.Provider
	redis  rueidis.Client
}

// NewCachedMinimaxClient creates a new CachedMinimaxClient.
func NewCachedMinimaxClient(client MinimaxClient, db db.Provider, redisClient rueidis.Client) MinimaxClient {
	return &CachedMinimaxClient{
		client: client,
		db:     db,
		redis:  redisClient,
	}
}

// Reason caches calls to the underlying Reason method.
func (c *CachedMinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	hashBytes := sha256.Sum256([]byte(prompt))
	promptHash := hex.EncodeToString(hashBytes[:])

	// 1. Try Redis cache
	if c.redis != nil {
		redisKey := "reason:" + promptHash
		cmd := c.redis.B().Get().Key(redisKey).Build()
		val, err := c.redis.Do(ctx, cmd).ToString()
		if err == nil && val != "" {
			slog.Debug("CachedMinimaxClient: found reason in Redis", "hash", promptHash)
			return val, nil
		}
	}

	// 2. Try DB cache
	if c.db != nil {
		var cachedResponse string
		err := c.db.QueryRow(ctx, "SELECT response FROM reason_cache WHERE prompt_hash = $1", promptHash).Scan(&cachedResponse)
		if err == nil && cachedResponse != "" {
			slog.Debug("CachedMinimaxClient: found reason in DB", "hash", promptHash)

			// Optional: populate Redis if it was missing there
			if c.redis != nil {
				cmd := c.redis.B().Set().Key("reason:" + promptHash).Value(cachedResponse).Ex(24 * time.Hour).Build()
				_ = c.redis.Do(ctx, cmd)
			}
			return cachedResponse, nil
		}
	}

	// 3. Cache miss: generate reason
	response, err := c.client.Reason(ctx, prompt)
	if err != nil {
		return "", err
	}

	// Save to Redis
	if c.redis != nil {
		cmd := c.redis.B().Set().Key("reason:" + promptHash).Value(response).Ex(24 * time.Hour).Build()
		if err := c.redis.Do(ctx, cmd).Error(); err != nil {
			slog.Warn("CachedMinimaxClient: failed to save reason to Redis", "err", err)
		}
	}

	// Save to DB
	if c.db != nil {
		cacheQuery := "INSERT INTO reason_cache (prompt_hash, response) VALUES ($1, $2) ON CONFLICT (prompt_hash) DO NOTHING"
		if c.db.IsSQLite() {
			cacheQuery = "INSERT INTO reason_cache (prompt_hash, response) VALUES (?, ?) ON CONFLICT (prompt_hash) DO NOTHING"
		}
		_, cacheErr := c.db.Exec(ctx, cacheQuery, promptHash, response)
		if cacheErr != nil {
			slog.Warn("CachedMinimaxClient: failed to save reason to DB cache", "err", cacheErr)
		}
	}

	return response, nil
}

// GenerateEmbedding caches calls to the underlying GenerateEmbedding.
func (c *CachedMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	hashBytes := sha256.Sum256([]byte(text))
	contentHash := hex.EncodeToString(hashBytes[:])

	// 1. Try Redis cache
	if c.redis != nil {
		redisKey := "embedding:" + contentHash
		cmd := c.redis.B().Get().Key(redisKey).Build()
		val, err := c.redis.Do(ctx, cmd).ToString()
		if err == nil && val != "" {
			var embedding []float32
			if err := json.Unmarshal([]byte(val), &embedding); err == nil {
				slog.Debug("CachedMinimaxClient: found embedding in Redis", "hash", contentHash)
				return embedding, nil
			}
		}
	}

	// 2. Try DB cache
	if c.db != nil {
		var cachedEmbeddingStr string
		err := c.db.QueryRow(ctx, "SELECT embedding FROM embedding_cache WHERE content_hash = $1", contentHash).Scan(&cachedEmbeddingStr)
		if err == nil && cachedEmbeddingStr != "" {
			var embedding []float32
			if err := json.Unmarshal([]byte(cachedEmbeddingStr), &embedding); err == nil {
				slog.Debug("CachedMinimaxClient: found embedding in DB", "hash", contentHash)

				// Optional: populate Redis if it was missing there
				if c.redis != nil {
					cmd := c.redis.B().Set().Key("embedding:" + contentHash).Value(cachedEmbeddingStr).Ex(24 * time.Hour).Build()
					_ = c.redis.Do(ctx, cmd)
				}
				return embedding, nil
			}
		}
	}

	// 3. Cache miss: generate embedding
	embedding, err := c.client.GenerateEmbedding(ctx, text)
	if err != nil {
		return nil, err
	}

	embeddingBytes, err := json.Marshal(embedding)
	if err == nil {
		embeddingStr := string(embeddingBytes)

		// Save to Redis
		if c.redis != nil {
			cmd := c.redis.B().Set().Key("embedding:" + contentHash).Value(embeddingStr).Ex(24 * time.Hour).Build()
			if err := c.redis.Do(ctx, cmd).Error(); err != nil {
				slog.Warn("CachedMinimaxClient: failed to save embedding to Redis", "err", err)
			}
		}

		// Save to DB
		if c.db != nil {
			cacheQuery := "INSERT INTO embedding_cache (content_hash, embedding) VALUES ($1, $2) ON CONFLICT (content_hash) DO NOTHING"
			if c.db.IsSQLite() {
				cacheQuery = "INSERT INTO embedding_cache (content_hash, embedding) VALUES (?, ?) ON CONFLICT (content_hash) DO NOTHING"
			}
			_, cacheErr := c.db.Exec(ctx, cacheQuery, contentHash, embeddingStr)
			if cacheErr != nil {
				slog.Warn("CachedMinimaxClient: failed to save embedding to DB cache", "err", cacheErr)
			}
		}
	}

	return embedding, nil
}
