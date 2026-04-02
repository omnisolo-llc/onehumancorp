package orchestration

import (
	"context"
	"encoding/json"
	"log/slog"
	"strings"
	"time"
)

// AutoDreamSystem handles memory consolidation and stale data pruning.
// It acts as the background worker pipeline for vector embeddings.
type AutoDreamSystem struct {
	sipdb *SIPDB
}

// NewAutoDreamSystem creates a new AutoDreamSystem.
func NewAutoDreamSystem(sipdb *SIPDB) *AutoDreamSystem {
	return &AutoDreamSystem{
		sipdb: sipdb,
	}
}

// Start initiates the background worker loops for AutoDream.
func (a *AutoDreamSystem) Start(ctx context.Context) {
	// Start Memory Pruning Pipeline
	go a.runPruningPipeline(ctx)
	// Start Memory Consolidation & Conflict Resolution Pipeline
	go a.runConsolidationPipeline(ctx)
}

func (a *AutoDreamSystem) runPruningPipeline(ctx context.Context) {
	ticker := time.NewTicker(10 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			// Periodically prune stale agent session data (e.g., old memory keys that are transient)
			// For this implementation we will consider swarm_memory keys older than 30 days as transient and prune them.
			a.pruneTransientContext(ctx)
		}
	}
}

func (a *AutoDreamSystem) pruneTransientContext(ctx context.Context) {
	err := withRetry(ctx, func() error {
		thresholdTime := time.Now().Add(-30 * 24 * time.Hour).UTC().Format("2006-01-02 15:04:05")
		_, err := a.sipdb.db.Exec(ctx, "DELETE FROM swarm_memory WHERE updated_at < ?", thresholdTime)
		if err == nil {
			slog.Info("AutoDream: pruned transient swarm memory")
		}
		return err
	})
	if err != nil {
		slog.Error("AutoDream: failed to prune transient context", "error", err)
	}
}

func (a *AutoDreamSystem) runConsolidationPipeline(ctx context.Context) {
	ticker := time.NewTicker(15 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			// Consolidate swarm_memory entries into swarm_memory_embeddings
			a.consolidateMemories(ctx)
		}
	}
}

func (a *AutoDreamSystem) consolidateMemories(ctx context.Context) {
	var transientMemories []struct {
		Key   string
		Value string
	}

	err := withRetry(ctx, func() error {
		rows, err := a.sipdb.db.Query(ctx, "SELECT key, value FROM swarm_memory LIMIT 50")
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var k, v string
			if err := rows.Scan(&k, &v); err != nil {
				return err
			}
			transientMemories = append(transientMemories, struct {
				Key   string
				Value string
			}{k, v})
		}
		return nil
	})

	if err != nil {
		slog.Error("AutoDream: failed to fetch transient memories for consolidation", "error", err)
		return
	}

	for _, tm := range transientMemories {
		// Mock logic for embedding generation / truth injection
		vector := generateMockEmbedding(tm.Value)

		// Check for conflicts in existing long-term memory
		resolvedContext := a.resolveConflicts(ctx, tm.Value, vector)

		// Store consolidated memory
		memory := EpisodicMemory{
			MemoryID:        "consolidated-" + tm.Key,
			Context:         resolvedContext,
			VectorEmbedding: vector,
			SourcePlugin:    "autodream-consolidation",
		}

		err = a.sipdb.StoreEpisodicMemory(ctx, memory)
		if err != nil {
			slog.Error("AutoDream: failed to store episodic memory", "key", tm.Key, "error", err)
			continue
		}

		// Prune the original transient memory since it's consolidated
		_ = withRetry(ctx, func() error {
			_, e := a.sipdb.db.Exec(ctx, "DELETE FROM swarm_memory WHERE key = ?", tm.Key)
			return e
		})
	}
}

func (a *AutoDreamSystem) resolveConflicts(ctx context.Context, newContext string, vector []byte) string {
	// Conflict Resolution Logic:
	// In a real system, we would query the pgvector database using vector similarity search (e.g. `<->` operator in Postgres).
	// Here, we simulate detecting a contradiction and resolving it by prepending "Resolved: ".
	// Since we need to run in both SQLite (Standalone) and Postgres (Cloud-Native), we abstract this search.

	var existingMemories []EpisodicMemory
	_ = withRetry(ctx, func() error {
		// Mock simple similarity search by fetching some memories
		rows, err := a.sipdb.db.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings LIMIT 5")
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var m EpisodicMemory
			var t string
			if err := rows.Scan(&m.MemoryID, &m.Context, &m.VectorEmbedding, &m.SourcePlugin, &t); err == nil {
				existingMemories = append(existingMemories, m)
			}
		}
		return nil
	})

	resolvedContext := newContext
	for _, em := range existingMemories {
		// If existing context shares keywords, we simulate a conflict resolution merge
		if strings.Contains(em.Context, "conflict-trigger") && strings.Contains(newContext, "conflict-trigger") {
			resolvedContext = "Resolved Knowledge: [" + em.Context + " | " + newContext + "]"
			break
		}
	}

	return resolvedContext
}

func generateMockEmbedding(text string) []byte {
	// Truth Injection: Generates high-dimensional semantic memory representation (mocked)
	// Real implementation would call an LLM embedding API (e.g., text-embedding-ada-002)
	payload, _ := json.Marshal(map[string]string{"type": "vector", "source": text})
	return payload
}
