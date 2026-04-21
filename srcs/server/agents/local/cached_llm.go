package local

import (
	"math/rand"

	"bytes"
	"compress/gzip"
	"context"
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"io"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// compressData compresses the given byte slice using gzip.
func compressData(data []byte) ([]byte, error) {
	var b bytes.Buffer
	w := gzip.NewWriter(&b)
	if _, err := w.Write(data); err != nil {
		return nil, err
	}
	if err := w.Close(); err != nil {
		return nil, err
	}
	return b.Bytes(), nil
}

// decompressData decompresses the underlying byte slice using gzip.
// If the data is not valid base64 or not a valid gzip stream, it attempts to fall back.
// Since we previously used base64, we will check if it's base64 encoded for backwards compatibility
// with existing systems, but going forward it will be raw compressed bytes.
func decompressData(data []byte) ([]byte, error) {
	if len(data) == 0 {
		return data, nil
	}

	decodedBytes := data

	// Fast check for gzip magic number (0x1F, 0x8B)
	if len(decodedBytes) < 2 || decodedBytes[0] != 0x1f || decodedBytes[1] != 0x8b {
		// Not gzip, could be base64 from a previous run or uncompressed json.
		// Let's try base64 decode just in case.
		b64Decoded := make([]byte, base64.StdEncoding.DecodedLen(len(data)))
		n, err := base64.StdEncoding.Decode(b64Decoded, data)
		if err == nil && n >= 2 && b64Decoded[0] == 0x1f && b64Decoded[1] == 0x8b {
			decodedBytes = b64Decoded[:n]
		} else {
			// Not base64 gzip either, assume it's raw text.
			return data, nil
		}
	}

	r, err := gzip.NewReader(bytes.NewReader(decodedBytes))
	if err != nil {
		return data, nil
	}
	defer r.Close()

	decompressed, err := io.ReadAll(r)
	if err != nil {
		return data, nil
	}
	return decompressed, nil
}

// PruneCache periodically removes DB cache entries older than 24 hours to prevent unbounded local storage growth.
func (c *CachedLLMClient) PruneCache(ctx context.Context) {
	if c.db == nil {
		return
	}

	query := "DELETE FROM llm_completion_cache WHERE created_at < NOW() - INTERVAL '1 day'"
	if c.db.IsSQLite() {
		query = "DELETE FROM llm_completion_cache WHERE created_at < datetime('now', '-1 day')"
	}

	_, err := c.db.Exec(ctx, query)
	if err != nil {
		slog.Warn("CachedLLMClient: failed to prune llm_completion_cache", "err", err)
	}
}

// CachedLLMClient wraps an LLMClient and caches AssistantMessage responses
// in Redis (if available) and the DB llm_completion_cache table.
type CachedLLMClient struct {
	client LLMClient
	db     db.Provider
	redis  rueidis.Client
}

// NewCachedLLMClient creates a new CachedLLMClient.
func NewCachedLLMClient(client LLMClient, db db.Provider, redisClient rueidis.Client) LLMClient {
	return &CachedLLMClient{
		client: client,
		db:     db,
		redis:  redisClient,
	}
}

// Complete caches calls to the underlying Complete method.
func (c *CachedLLMClient) Complete(ctx context.Context, req CompletionRequest) (*AssistantMessage, error) {
	// 1. Hash the request
	reqBytes, err := json.Marshal(req)
	if err != nil {
		// If we can't marshal it, just call the client directly
		return c.client.Complete(ctx, req)
	}
	hashBytes := sha256.Sum256(reqBytes)
	reqHash := hex.EncodeToString(hashBytes[:])

	// 2. Try Redis cache
	if c.redis != nil {
		redisKey := "llm_completion:" + reqHash
		cmd := c.redis.B().Get().Key(redisKey).Build()
		val, err := c.redis.Do(ctx, cmd).AsBytes()
		if err == nil && len(val) > 0 {
			decompressed, err := decompressData(val)
			if err == nil {
				var msg AssistantMessage
				if err := json.Unmarshal(decompressed, &msg); err == nil {
					slog.Debug("CachedLLMClient: found completion in Redis", "hash", reqHash)
					telemetry.RecordCacheHit(ctx, "llm_completion", "redis")
					return &msg, nil
				}
			}
		}
	}

	// 3. Try DB cache
	if c.db != nil {
		var cachedResponse []byte
		selectQuery := "SELECT response_payload FROM llm_completion_cache WHERE request_hash = $1"
		if c.db.IsSQLite() {
			selectQuery = "SELECT response_payload FROM llm_completion_cache WHERE request_hash = ?"
		}
		err := c.db.QueryRow(ctx, selectQuery, reqHash).Scan(&cachedResponse)
		if err == nil && len(cachedResponse) > 0 {
			decompressed, err := decompressData(cachedResponse)
			if err == nil {
				var msg AssistantMessage
				if err := json.Unmarshal(decompressed, &msg); err == nil {
					slog.Debug("CachedLLMClient: found completion in DB", "hash", reqHash)
					telemetry.RecordCacheHit(ctx, "llm_completion", "db")

					// Optional: populate Redis if it was missing there
					if c.redis != nil {
						cmd := c.redis.B().Set().Key("llm_completion:" + reqHash).Value(string(cachedResponse)).Ex(24 * time.Hour).Build()
						_ = c.redis.Do(ctx, cmd)
					}
					return &msg, nil
				}
			}
		}
	}

	// 4. Cache miss: generate response
	telemetry.RecordCacheMiss(ctx, "llm_completion", "all")

	if rand.Float32() < 0.01 {
		go c.PruneCache(context.Background())
	}
	resp, err := c.client.Complete(ctx, req)
	if err != nil {
		return nil, err
	}

	// Cache the response
	respBytes, err := json.Marshal(resp)
	if err == nil {
		compressedBytes, err := compressData(respBytes)
		if err != nil {
			slog.Warn("CachedLLMClient: failed to compress response", "err", err)
			compressedBytes = respBytes
		}

		// Save to Redis
		if c.redis != nil {
			cmd := c.redis.B().Set().Key("llm_completion:" + reqHash).Value(string(compressedBytes)).Ex(24 * time.Hour).Build()
			if err := c.redis.Do(ctx, cmd).Error(); err != nil {
				slog.Warn("CachedLLMClient: failed to save completion to Redis", "err", err)
			}
		}

		// Save to DB
		if c.db != nil {
			cacheQuery := "INSERT INTO llm_completion_cache (request_hash, response_payload) VALUES ($1, $2) ON CONFLICT (request_hash) DO NOTHING"
			if c.db.IsSQLite() {
				cacheQuery = "INSERT INTO llm_completion_cache (request_hash, response_payload) VALUES (?, ?) ON CONFLICT (request_hash) DO NOTHING"
			}
			_, cacheErr := c.db.Exec(ctx, cacheQuery, reqHash, compressedBytes)
			if cacheErr != nil {
				slog.Warn("CachedLLMClient: failed to save completion to DB cache", "err", cacheErr)
			}
		}
	}

	return resp, nil
}
