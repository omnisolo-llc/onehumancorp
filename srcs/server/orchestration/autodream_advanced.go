package orchestration

import (
    "context"
    "fmt"
    "log/slog"
    "time"
    "strings"

    "github.com/google/uuid"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type AutoDreamAdvanced struct {
    pool   db.Provider
    client MinimaxClient
}

func NewAutoDreamAdvanced(pool db.Provider, client MinimaxClient) *AutoDreamAdvanced {
    return &AutoDreamAdvanced{pool: pool, client: client}
}

func (a *AutoDreamAdvanced) PruneStaleAgentSessions(ctx context.Context) error {
    var query string
    if a.pool.IsSQLite() {
        query = "DELETE FROM agent_session_data WHERE last_accessed < datetime('now', '-30 days')"
    } else {
        query = "DELETE FROM agent_session_data WHERE last_accessed < CURRENT_TIMESTAMP - INTERVAL '30 days'"
    }
    res, err := a.pool.Exec(ctx, query)
    if err != nil {
        return fmt.Errorf("failed to prune stale agent sessions: %w", err)
    }
    slog.Info("AutoDreamAdvanced: Pruned stale agent sessions", "deleted_rows", res)
    return nil
}

func (a *AutoDreamAdvanced) ResolveConflicts(ctx context.Context, memoryID string) error {
    var query string
    if a.pool.IsSQLite() {
        query = `SELECT id, content FROM autodream_memories WHERE id != $1 LIMIT 5`
    } else {
        // Find close neighbors to check for conflicts
        query = `SELECT id, content FROM autodream_memories WHERE id != $1 ORDER BY embedding <-> (SELECT embedding FROM autodream_memories WHERE id = $1) ASC LIMIT 5`
    }

    rows, err := a.pool.Query(ctx, query, memoryID)
    if err != nil {
        return err
    }
    defer rows.Close()

    var conflictIDs []string
    var contents []string
    for rows.Next() {
        var id, content string
        if err := rows.Scan(&id, &content); err == nil {
            conflictIDs = append(conflictIDs, id)
            contents = append(contents, content)
        }
    }

    if len(conflictIDs) > 0 {
        // Instead of blindly deleting, we would use an LLM to resolve.
        // For now, we simulate a resolved synthesis.
        resolvedContent := "Resolved knowledge from conflicts: " + strings.Join(contents, " | ")

        // Since we don't have a completion method on MinimaxClient interface right now,
        // we'll just inject the merged truth and delete the old ones.
        err := a.InjectTruth(ctx, "system", "auto-dream-resolver", resolvedContent)
        if err != nil {
             return fmt.Errorf("failed to inject resolved truth: %w", err)
        }

        for _, id := range conflictIDs {
            _, _ = a.pool.Exec(ctx, "DELETE FROM autodream_memories WHERE id = $1", id)
        }
        _, _ = a.pool.Exec(ctx, "DELETE FROM autodream_memories WHERE id = $1", memoryID)

        slog.Info("AutoDreamAdvanced: Resolved conflicts", "deleted_ids", conflictIDs, "merged", resolvedContent)
    }
    return nil
}

func (a *AutoDreamAdvanced) InjectTruth(ctx context.Context, orgID, agentID, content string) error {
    embedding := make([]float32, 1536)
    if a.client != nil {
        ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
        resp, err := a.client.GenerateEmbedding(ctxTimeout, content)
        cancel()
        if err == nil && len(resp) == 1536 {
            embedding = resp
        }
    }
    embStr := formatFloat32SliceForVector(embedding)
    id := uuid.New().String()

    var query string
    if a.pool.IsSQLite() {
        query = `INSERT INTO autodream_memories (id, content, embedding, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3, $4, $5, 'truth', CURRENT_TIMESTAMP)`
    } else {
        query = `INSERT INTO autodream_memories (id, content, embedding, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3::vector, $4, $5, 'truth', CURRENT_TIMESTAMP)`
    }
    _, err := a.pool.Exec(ctx, query, id, content, embStr, orgID, agentID)
    if err != nil {
        return fmt.Errorf("failed to inject truth: %w", err)
    }
    slog.Info("AutoDreamAdvanced: Injected truth", "id", id)
    return nil
}

func (a *AutoDreamAdvanced) SearchTruth(ctx context.Context, orgID, queryText string, limit int) ([]string, error) {
    embedding := make([]float32, 1536)
    if a.client != nil {
        ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
        resp, err := a.client.GenerateEmbedding(ctxTimeout, queryText)
        cancel()
        if err == nil && len(resp) == 1536 {
            embedding = resp
        }
    }
    embStr := formatFloat32SliceForVector(embedding)

    var query string
    if a.pool.IsSQLite() {
        query = `SELECT content FROM autodream_memories WHERE organization_id = $1 AND source_type = 'truth' LIMIT $2`
    } else {
        query = `SELECT content FROM autodream_memories WHERE organization_id = $1 AND source_type = 'truth' ORDER BY embedding <-> $2::vector ASC LIMIT $3`
    }

    var rows db.Rows
    var err error
    if a.pool.IsSQLite() {
        rows, err = a.pool.Query(ctx, query, orgID, limit)
    } else {
        rows, err = a.pool.Query(ctx, query, orgID, embStr, limit)
    }
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var results []string
    for rows.Next() {
        var content string
        if err := rows.Scan(&content); err == nil {
            results = append(results, content)
        }
    }
    return results, nil
}

// AutoDreamAdvancedDaemon runs background pipelines.
type AutoDreamAdvancedDaemon struct {
	advanced *AutoDreamAdvanced
	done     chan struct{}
}

func NewAutoDreamAdvancedDaemon(advanced *AutoDreamAdvanced) *AutoDreamAdvancedDaemon {
	return &AutoDreamAdvancedDaemon{
		advanced: advanced,
		done:     make(chan struct{}),
	}
}

func (d *AutoDreamAdvancedDaemon) Start(ctx context.Context) {
	ticker := time.NewTicker(24 * time.Hour)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			d.Stop()
			return
		case <-d.done:
			return
		case <-ticker.C:
			if err := d.advanced.PruneStaleAgentSessions(ctx); err != nil {
				slog.Error("AutoDreamAdvancedDaemon: failed to prune sessions", "error", err)
			}
		}
	}
}

func (d *AutoDreamAdvancedDaemon) Stop() {
	close(d.done)
}
