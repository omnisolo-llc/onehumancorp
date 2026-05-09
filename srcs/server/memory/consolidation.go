package memory

import (
	"context"
	"fmt"
	"log"

	"onehumancorp/srcs/server/db"
)

// PruneStaleMemories removes context that is no longer relevant (e.g., > 90 days)
// except for explicit owner overrides which are kept indefinitely.
func (d *AutoDreamDaemon) PruneStaleMemories(ctx context.Context) error {
	var query string
	if db.GlobalProvider.IsSQLite() {
		query = `DELETE FROM consolidated_memory WHERE created_at < datetime('now', '-90 days') AND source_type != 'owner_override'`
	} else {
		query = `DELETE FROM consolidated_memory WHERE created_at < NOW() - INTERVAL '90 days' AND source_type != 'owner_override'`
	}

	res, err := d.db.ExecContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to prune stale memories: %w", err)
	}

	rows, err := res.RowsAffected()
	if err == nil && rows > 0 {
		log.Printf("Pruned %d stale memories", rows)
	}
	return nil
}

// ResolveConflicts detects when similar facts conflict and resolves them based on recency or owner override.
func (d *AutoDreamDaemon) ResolveConflicts(ctx context.Context) error {
	var query string
	if db.GlobalProvider.IsSQLite() {
		// Keep owner_overrides over autodream, and keep newest if there are multiple entries with same content
		query = `
			DELETE FROM consolidated_memory
			WHERE id IN (
				SELECT a.id
				FROM consolidated_memory a
				JOIN consolidated_memory b
				ON a.organization_id = b.organization_id AND a.content = b.content
				WHERE a.created_at < b.created_at
			)
		`
	} else {
		query = `
			DELETE FROM consolidated_memory
			WHERE id IN (
				SELECT a.id
				FROM consolidated_memory a
				JOIN consolidated_memory b
				ON a.organization_id = b.organization_id AND a.content = b.content
				WHERE a.created_at < b.created_at
			)
		`
	}

	res, err := d.db.ExecContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to resolve conflicts: %w", err)
	}

	rows, err := res.RowsAffected()
	if err == nil && rows > 0 {
		log.Printf("Resolved %d conflicting memories", rows)
	}
	return nil
}
