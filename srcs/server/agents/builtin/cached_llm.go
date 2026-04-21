package builtin

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"log/slog"
	"math/rand"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// CachedLLMClient wraps an LLMClient and caches responses in Redis/DB.
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

func (c *CachedLLMClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	// 1. Hash the request
	reqBytes, err := json.Marshal(req)
	if err != nil {
		return c.client.Chat(ctx, req)
	}
	hashBytes := sha256.Sum256(reqBytes)
	reqHash := hex.EncodeToString(hashBytes[:])

	// 2. Try Redis cache
	if c.redis != nil {
		redisKey := "llm_chat:" + reqHash
		cmd := c.redis.B().Get().Key(redisKey).Build()
		val, err := c.redis.Do(ctx, cmd).AsBytes()
		if err == nil && len(val) > 0 {
			var resp ChatResponse
			if err := json.Unmarshal(val, &resp); err == nil {
				slog.Debug("CachedLLMClient: found response in Redis", "hash", reqHash)
				telemetry.RecordCacheHit(ctx, "llm_chat", "redis")
				return resp, nil
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
			var resp ChatResponse
			if err := json.Unmarshal(cachedResponse, &resp); err == nil {
				slog.Debug("CachedLLMClient: found response in DB", "hash", reqHash)
				telemetry.RecordCacheHit(ctx, "llm_chat", "db")

				if c.redis != nil {
					cmd := c.redis.B().Set().Key("llm_chat:" + reqHash).Value(string(cachedResponse)).Ex(24 * time.Hour).Build()
					_ = c.redis.Do(ctx, cmd)
				}
				return resp, nil
			}
		}
	}

	// 4. Cache miss
	telemetry.RecordCacheMiss(ctx, "llm_chat", "all")

	if rand.Float32() < 0.01 {
		go c.PruneCache(context.Background())
	}

	resp, err := c.client.Chat(ctx, req)
	if err != nil {
		return ChatResponse{}, err
	}

	// Cache the response
	respBytes, err := json.Marshal(resp)
	if err == nil {
		if c.redis != nil {
			cmd := c.redis.B().Set().Key("llm_chat:" + reqHash).Value(string(respBytes)).Ex(24 * time.Hour).Build()
			_ = c.redis.Do(ctx, cmd)
		}
		if c.db != nil {
			cacheQuery := "INSERT INTO llm_completion_cache (request_hash, response_payload) VALUES ($1, $2) ON CONFLICT (request_hash) DO NOTHING"
			if c.db.IsSQLite() {
				cacheQuery = "INSERT INTO llm_completion_cache (request_hash, response_payload) VALUES (?, ?) ON CONFLICT (request_hash) DO NOTHING"
			}
			_, _ = c.db.Exec(ctx, cacheQuery, reqHash, respBytes)
		}
	}

	return resp, nil
}

func (c *CachedLLMClient) PruneCache(ctx context.Context) {
	if c.db == nil {
		return
	}
	query := "DELETE FROM llm_completion_cache WHERE created_at < NOW() - INTERVAL '1 day'"
	if c.db.IsSQLite() {
		query = "DELETE FROM llm_completion_cache WHERE created_at < datetime('now', '-1 day')"
	}
	_, _ = c.db.Exec(ctx, query)
}
