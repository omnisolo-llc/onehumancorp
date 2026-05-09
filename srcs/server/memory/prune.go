package memory

import (
	"context"
	"log"
	"time"
	"onehumancorp/srcs/server/db"
	"fmt"
)

// PruneStaleMemories deletes memories that have not been accessed recently.
// Pruning is extremely conservative, we only prune purely background contexts.
func (d *AutoDreamDaemon) PruneStaleMemories(ctx context.Context, threshold time.Duration) error {
	// Delete where last_accessed_at is older than the threshold, but only for purely non-critical background facts.
	var query string
	if db.GlobalProvider != nil && db.GlobalProvider.IsSQLite() {
		query = `
			DELETE FROM consolidated_memory
			WHERE COALESCE(last_accessed_at, created_at) < ?
			  AND source_type = 'autodream_background'
		`
	} else {
		query = `
			DELETE FROM consolidated_memory
			WHERE COALESCE(last_accessed_at, created_at) < $1
			  AND source_type = 'autodream_background'
		`
	}
	cutoff := time.Now().Add(-threshold)

	result, err := d.db.ExecContext(ctx, query, cutoff)
	if err != nil {
		return err
	}

	if rowsAffected, err := result.RowsAffected(); err == nil && rowsAffected > 0 {
		log.Printf("Pruned %d stale memories", rowsAffected)
	}

	return nil
}

// ResolveConflicts resolves semantic conflicts by checking similarity.
// We keep the most recent memory.
func (d *AutoDreamDaemon) ResolveConflicts(ctx context.Context) error {
	// To avoid O(N^2) DOS, we ONLY resolve exact string duplicates within the same tenant.
	// True conflict resolution across semantic facts requires an LLM call to synthesize the facts.
	// For this background worker, we just clean up exact duplicates to prevent database bloat
	// until a proper LLM-based consolidation step is implemented.

	query := `
		DELETE FROM consolidated_memory
		WHERE id IN (
			SELECT a.id
			FROM consolidated_memory a
			INNER JOIN consolidated_memory b
			ON a.organization_id = b.organization_id
			AND a.id != b.id
			AND a.created_at < b.created_at
			AND a.content = b.content
		)
	`

	result, err := d.db.ExecContext(ctx, query)
	if err != nil {
		return fmt.Errorf("conflict resolution query failed: %w", err)
	}

	if rowsAffected, err := result.RowsAffected(); err == nil && rowsAffected > 0 {
		log.Printf("Resolved %d conflicting memories (exact match duplicates)", rowsAffected)
	}

	return nil
}
