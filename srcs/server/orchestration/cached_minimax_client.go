package orchestration

import (
	"bytes"
	"compress/gzip"
	"context"
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"io"
	"math/rand"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// compressData compresses the given byte slice using gzip and encodes it to base64 string.
func compressData(data []byte) ([]byte, error) {
	var b bytes.Buffer
	w := gzip.NewWriter(&b)
	_, err := w.Write(data)
	if err != nil {
		return nil, err
	}
	err = w.Close()
	if err != nil {
		return nil, err
	}

	encoded := base64.StdEncoding.EncodeToString(b.Bytes())
	return []byte(encoded), nil
}

// decompressData decodes the base64 string and decompresses the underlying byte slice using gzip.
// If the data is not valid base64 or not a valid gzip stream, it assumes it's uncompressed plain text (backward compatibility).
func decompressData(data []byte) ([]byte, error) {
	// First, try decoding base64
	decodedBytes := make([]byte, base64.StdEncoding.DecodedLen(len(data)))
	n, err := base64.StdEncoding.Decode(decodedBytes, data)
	if err != nil {
		// Not valid base64, assume it's uncompressed
		return data, nil
	}
	decodedBytes = decodedBytes[:n]

	if len(decodedBytes) < 2 || decodedBytes[0] != 0x1f || decodedBytes[1] != 0x8b {
		// Not a valid gzip header, might be uncompressed data that just happened to look like base64
		// But usually it would fail Decode. To be safe, let's just return original data.
		return data, nil
	}

	r, err := gzip.NewReader(bytes.NewReader(decodedBytes))
	if err != nil {
		return data, nil // Return original data on fail to be safe
	}
	defer r.Close()

	decompressed, err := io.ReadAll(r)
	if err != nil {
		return data, nil
	}
	return decompressed, nil
}

// CachedMinimaxClient wraps a MinimaxClient and caches embeddings
// in Redis (if available) and the DB embedding_cache table.
type CachedMinimaxClient struct {
	client MinimaxClient
	db     db.Provider
	redis  rueidis.Client
}

// PruneCache periodically removes DB cache entries older than 24 hours to prevent unbounded local storage growth.
func (c *CachedMinimaxClient) PruneCache(ctx context.Context) {
	if c.db == nil {
		return
	}

	queryReason := "DELETE FROM llm_reason_cache WHERE created_at < NOW() - INTERVAL '1 day'"
	queryEmbed := "DELETE FROM embedding_cache WHERE created_at < NOW() - INTERVAL '1 day'"
	if c.db.IsSQLite() {
		queryReason = "DELETE FROM llm_reason_cache WHERE created_at < datetime('now', '-1 day')"
		queryEmbed = "DELETE FROM embedding_cache WHERE created_at < datetime('now', '-1 day')"
	}

	_, err := c.db.Exec(ctx, queryReason)
	if err != nil {
		slog.Warn("CachedMinimaxClient: failed to prune llm_reason_cache", "err", err)
	}
	_, err = c.db.Exec(ctx, queryEmbed)
	if err != nil {
		slog.Warn("CachedMinimaxClient: failed to prune embedding_cache", "err", err)
	}
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
		val, err := c.redis.Do(ctx, cmd).AsBytes()
		if err == nil && len(val) > 0 {
			decompressed, err := decompressData(val)
			if err == nil {
				slog.Debug("CachedMinimaxClient: found reason response in Redis", "hash", promptHash)
				telemetry.RecordCacheHit(ctx, "reason", "redis")
				telemetry.RecordTokensSaved(ctx, "reason", "redis", int64(len(prompt)/4))
				return string(decompressed), nil
			}
		}
	}

	// 2. Try DB cache
	if c.db != nil {
		var cachedResponse []byte
		selectQuery := "SELECT response FROM llm_reason_cache WHERE prompt_hash = $1"
		if c.db.IsSQLite() {
			selectQuery = "SELECT response FROM llm_reason_cache WHERE prompt_hash = ?"
		}
		err := c.db.QueryRow(ctx, selectQuery, promptHash).Scan(&cachedResponse)
		if err == nil && len(cachedResponse) > 0 {
			decompressed, err := decompressData(cachedResponse)
			if err == nil {
				slog.Debug("CachedMinimaxClient: found reason response in DB", "hash", promptHash)
				telemetry.RecordCacheHit(ctx, "reason", "db")
				telemetry.RecordTokensSaved(ctx, "reason", "db", int64(len(prompt)/4))

				// Optional: populate Redis if it was missing there
				if c.redis != nil {
					cmd := c.redis.B().Set().Key("llm_reason:" + promptHash).Value(string(cachedResponse)).Ex(24 * time.Hour).Build()
					_ = c.redis.Do(ctx, cmd)
				}
				return string(decompressed), nil
			}
		}
	}

	// 3. Cache miss: generate response
	telemetry.RecordCacheMiss(ctx, "reason", "all")

	if rand.Float32() < 0.01 {
		go c.PruneCache(context.Background())
	}

	response, err := c.client.Reason(ctx, prompt)
	if err != nil {
		return "", err
	}

	compressedResponse, err := compressData([]byte(response))
	if err != nil {
		slog.Warn("CachedMinimaxClient: failed to compress reason response", "err", err)
		compressedResponse = []byte(response)
	}

	// Save to Redis
	if c.redis != nil {
		cmd := c.redis.B().Set().Key("llm_reason:" + promptHash).Value(string(compressedResponse)).Ex(24 * time.Hour).Build()
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
		val, err := c.redis.Do(ctx, cmd).AsBytes()
		if err == nil && len(val) > 0 {
			decompressed, err := decompressData(val)
			if err == nil {
				var embedding []float32
				if err := json.Unmarshal(decompressed, &embedding); err == nil {
					slog.Debug("CachedMinimaxClient: found embedding in Redis", "hash", contentHash)
					telemetry.RecordCacheHit(ctx, "embedding", "redis")
					telemetry.RecordTokensSaved(ctx, "embedding", "redis", int64(len(text)/4))
					return embedding, nil
				}
			}
		}
	}

	// 2. Try DB cache
	if c.db != nil {
		var cachedEmbeddingBytes []byte
		selectQuery := "SELECT embedding FROM embedding_cache WHERE content_hash = $1"
		if c.db.IsSQLite() {
			selectQuery = "SELECT embedding FROM embedding_cache WHERE content_hash = ?"
		}
		err := c.db.QueryRow(ctx, selectQuery, contentHash).Scan(&cachedEmbeddingBytes)
		if err == nil && len(cachedEmbeddingBytes) > 0 {
			decompressed, err := decompressData(cachedEmbeddingBytes)
			if err == nil {
				var embedding []float32
				if err := json.Unmarshal(decompressed, &embedding); err == nil {
					slog.Debug("CachedMinimaxClient: found embedding in DB", "hash", contentHash)
					telemetry.RecordCacheHit(ctx, "embedding", "db")
					telemetry.RecordTokensSaved(ctx, "embedding", "db", int64(len(text)/4))

					// Optional: populate Redis if it was missing there
					if c.redis != nil {
						cmd := c.redis.B().Set().Key("embedding:" + contentHash).Value(string(cachedEmbeddingBytes)).Ex(24 * time.Hour).Build()
						_ = c.redis.Do(ctx, cmd)
					}
					return embedding, nil
				}
			}
		}
	}

	// 3. Cache miss: generate embedding
	telemetry.RecordCacheMiss(ctx, "embedding", "all")

	if rand.Float32() < 0.01 {
		go c.PruneCache(context.Background())
	}

	embedding, err := c.client.GenerateEmbedding(ctx, text)
	if err != nil {
		return nil, err
	}

	embeddingBytes, err := json.Marshal(embedding)
	if err == nil {
		compressedBytes, err := compressData(embeddingBytes)
		if err != nil {
			slog.Warn("CachedMinimaxClient: failed to compress embedding", "err", err)
			compressedBytes = embeddingBytes
		}

		// Save to Redis
		if c.redis != nil {
			cmd := c.redis.B().Set().Key("embedding:" + contentHash).Value(string(compressedBytes)).Ex(24 * time.Hour).Build()
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
			_, cacheErr := c.db.Exec(ctx, cacheQuery, contentHash, compressedBytes)
			if cacheErr != nil {
				slog.Warn("CachedMinimaxClient: failed to save embedding to DB cache", "err", cacheErr)
			}
		}
	}

	return embedding, nil
}
