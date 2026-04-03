package orchestration

import (
	"bytes"
	"compress/gzip"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"io"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// compressString compresses a string using gzip and encodes to base64
func compressString(s string) (string, error) {
	var b bytes.Buffer
	w := gzip.NewWriter(&b)
	if _, err := w.Write([]byte(s)); err != nil {
		return "", err
	}
	if err := w.Close(); err != nil {
		return "", err
	}
	// Encode to hex/base64 to be safe for Postgres TEXT columns
	return hex.EncodeToString(b.Bytes()), nil
}

// decompressString decodes from hex and decompresses a gzip compressed string
func decompressString(s string) (string, error) {
	decoded, err := hex.DecodeString(s)
	if err != nil {
		return "", err
	}
	r, err := gzip.NewReader(bytes.NewReader(decoded))
	if err != nil {
		return "", err
	}
	defer r.Close()
	decompressed, err := io.ReadAll(r)
	if err != nil {
		return "", err
	}
	return string(decompressed), nil
}

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
		redisKey := "llm_reason:" + promptHash
		cmd := c.redis.B().Get().Key(redisKey).Build()
		val, err := c.redis.Do(ctx, cmd).ToString()
		if err == nil && val != "" {
			slog.Debug("CachedMinimaxClient: found reason response in Redis", "hash", promptHash)
			telemetry.RecordCacheHit(ctx, "reason", "redis")
			decompressed, err := decompressString(val)
			if err == nil {
				return decompressed, nil
			}
			slog.Warn("CachedMinimaxClient: failed to decompress Redis cached reason", "err", err)
			// fallback to return uncompressed just in case it was stored before compression was added
			return val, nil
		}
	}

	// 2. Try DB cache
	if c.db != nil {
		var cachedResponse string
		selectQuery := "SELECT response FROM llm_reason_cache WHERE prompt_hash = $1"
		if c.db.IsSQLite() {
			selectQuery = "SELECT response FROM llm_reason_cache WHERE prompt_hash = ?"
		}
		err := c.db.QueryRow(ctx, selectQuery, promptHash).Scan(&cachedResponse)
		if err == nil && cachedResponse != "" {
			slog.Debug("CachedMinimaxClient: found reason response in DB", "hash", promptHash)
			telemetry.RecordCacheHit(ctx, "reason", "db")

			// Optional: populate Redis if it was missing there
			if c.redis != nil {
				cmd := c.redis.B().Set().Key("llm_reason:" + promptHash).Value(cachedResponse).Ex(24 * time.Hour).Build()
				_ = c.redis.Do(ctx, cmd)
			}

			decompressed, err := decompressString(cachedResponse)
			if err == nil {
				return decompressed, nil
			}
			slog.Warn("CachedMinimaxClient: failed to decompress DB cached reason", "err", err)
			return cachedResponse, nil
		}
	}

	// 3. Cache miss: generate response
	telemetry.RecordCacheMiss(ctx, "reason", "all")
	response, err := c.client.Reason(ctx, prompt)
	if err != nil {
		return "", err
	}

	compressedResponse, err := compressString(response)
	if err != nil {
		slog.Warn("CachedMinimaxClient: failed to compress reason response", "err", err)
		compressedResponse = response // Fallback to raw response
	}

	// Save to Redis
	if c.redis != nil {
		cmd := c.redis.B().Set().Key("llm_reason:" + promptHash).Value(compressedResponse).Ex(24 * time.Hour).Build()
		if err := c.redis.Do(ctx, cmd).Error(); err != nil {
			slog.Warn("CachedMinimaxClient: failed to save reason response to Redis", "err", err)
		}
	}

	// Save to DB
	if c.db != nil {
		cacheQuery := "INSERT INTO llm_reason_cache (prompt_hash, response) VALUES ($1, $2) ON CONFLICT (prompt_hash) DO NOTHING"
		if c.db.IsSQLite() {
			cacheQuery = "INSERT INTO llm_reason_cache (prompt_hash, response) VALUES (?, ?) ON CONFLICT (prompt_hash) DO NOTHING"
		}
		_, cacheErr := c.db.Exec(ctx, cacheQuery, promptHash, compressedResponse)
		if cacheErr != nil {
			slog.Warn("CachedMinimaxClient: failed to save reason response to DB cache", "err", cacheErr)
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
				telemetry.RecordCacheHit(ctx, "embedding", "redis")
				return embedding, nil
			}
		}
	}

	// 2. Try DB cache
	if c.db != nil {
		var cachedEmbeddingStr string
		selectQuery := "SELECT embedding FROM embedding_cache WHERE content_hash = $1"
		if c.db.IsSQLite() {
			selectQuery = "SELECT embedding FROM embedding_cache WHERE content_hash = ?"
		}
		err := c.db.QueryRow(ctx, selectQuery, contentHash).Scan(&cachedEmbeddingStr)
		if err == nil && cachedEmbeddingStr != "" {
			var embedding []float32
			if err := json.Unmarshal([]byte(cachedEmbeddingStr), &embedding); err == nil {
				slog.Debug("CachedMinimaxClient: found embedding in DB", "hash", contentHash)
				telemetry.RecordCacheHit(ctx, "embedding", "db")

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
	telemetry.RecordCacheMiss(ctx, "embedding", "all")
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
