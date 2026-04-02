package sync

import (
	"context"
	"fmt"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/onehumancorp/mono/srcs/server/db"
)

var (
	syncCompletedCount = promauto.NewCounter(prometheus.CounterOpts{
		Name: "sync_completed_count",
		Help: "The total number of successful AutoDream vector synchronizations",
	})
	syncFailedCount = promauto.NewCounter(prometheus.CounterOpts{
		Name: "sync_failed_count",
		Help: "The total number of failed AutoDream vector synchronizations",
	})
)

type AutoDreamSync struct {
	dbProvider db.Provider
	ticker     *time.Ticker
	done       chan bool
}

func NewAutoDreamSync(dbProvider db.Provider) *AutoDreamSync {
	return &AutoDreamSync{
		dbProvider: dbProvider,
		done:       make(chan bool),
	}
}

func (s *AutoDreamSync) Start(interval time.Duration) {
	s.ticker = time.NewTicker(interval)
	go func() {
		for {
			select {
			case <-s.done:
				s.ticker.Stop()
				return
			case <-s.ticker.C:
				s.ProcessForecastTick(context.Background())
			}
		}
	}()
}

func (s *AutoDreamSync) Stop() {
	if s.done != nil {
		s.done <- true
	}
}

// ProcessForecastTick is exported so tests can call it synchronously.
func (s *AutoDreamSync) ProcessForecastTick(ctx context.Context) {
	// Only run in SQLite mode (Standalone Desktop Mode)
	if !s.dbProvider.IsSQLite() {
		return
	}

	// 1. Sync embedding_cache
	err := s.syncEmbeddingCache(ctx)
	if err != nil {
		syncFailedCount.Inc()
		fmt.Printf("AutoDreamSync error: failed to sync embedding_cache: %v\n", err)
	} else {
		syncCompletedCount.Inc()
	}

	// 2. Sync agent_missions
	err = s.syncAgentMissions(ctx)
	if err != nil {
		syncFailedCount.Inc()
		fmt.Printf("AutoDreamSync error: failed to sync agent_missions: %v\n", err)
	} else {
		syncCompletedCount.Inc()
	}
}

func (s *AutoDreamSync) syncEmbeddingCache(ctx context.Context) error {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	rows, err := tx.Query(ctx, "SELECT content_hash, embedding FROM embedding_cache WHERE synced_to_cloud = false")
	if err != nil {
		return err
	}
	defer rows.Close()

	var hashesToUpdate []string
	for rows.Next() {
		var hash, embedding string
		if err := rows.Scan(&hash, &embedding); err != nil {
			return err
		}

		// Simulate network call to Cloud API
		err = s.simulateCloudSync("embedding_cache", hash, embedding)
		if err == nil {
			hashesToUpdate = append(hashesToUpdate, hash)
		}
	}

	for _, hash := range hashesToUpdate {
		_, err := tx.Exec(ctx, "UPDATE embedding_cache SET synced_to_cloud = true WHERE content_hash = $1", hash)
		if err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}

func (s *AutoDreamSync) syncAgentMissions(ctx context.Context) error {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Since agent_missions might have variable fields depending on other migrations,
	// we just query id for now to mark it synced.
	rows, err := tx.Query(ctx, "SELECT id FROM agent_missions WHERE synced_to_cloud = false")
	if err != nil {
		return err
	}
	defer rows.Close()

	var idsToUpdate []string
	for rows.Next() {
		var id string
		if err := rows.Scan(&id); err != nil {
			return err
		}

		// Simulate network call
		err = s.simulateCloudSync("agent_missions", id, "")
		if err == nil {
			idsToUpdate = append(idsToUpdate, id)
		}
	}

	for _, id := range idsToUpdate {
		_, err := tx.Exec(ctx, "UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1", id)
		if err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}

func (s *AutoDreamSync) simulateCloudSync(table, id, data string) error {
	// In a real implementation, this would POST to /api/v1/sync/autodream
	return nil
}
