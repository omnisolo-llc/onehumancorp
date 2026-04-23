package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("srcs/server/orchestration/autodream.go")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	strContent := string(content)

	strContent = strings.Replace(strContent, `	mode := kairos.GetMode()
	start := time.Now()

	defer func() {
		autodream_metrics.BatchProcessingDuration.WithLabelValues(mode, "ingestCompletedTasks").Observe(time.Since(start).Seconds())
	}()

	tx, txErr := w.pool.Begin(ctx)
	if txErr != nil {
		autodream_metrics.ConsolidationErrorsTotal.WithLabelValues(mode, "ingestCompletedTasks", "begin_tx_failed").Inc()
		slog.Error("AutoDream: failed to begin tx for completed tasks", "error", txErr)
		return
	}
	defer tx.Rollback(ctx)

	var query string
	if w.pool.IsSQLite() {
		query = "SELECT id, title, COALESCE(payload, '{}') FROM shared_tasks WHERE status = 'COMPLETED' LIMIT 500"
	} else {
		query = "SELECT id, title, COALESCE(payload, '{}') FROM shared_tasks WHERE status = 'COMPLETED' LIMIT 500 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)`, `	mode := kairos.GetMode()
	start := time.Now()

	defer func() {
		autodream_metrics.BatchProcessingDuration.WithLabelValues(mode, "ingestCompletedTasks").Observe(time.Since(start).Seconds())
	}()

	tx, txErr := w.pool.Begin(ctx)
	if txErr != nil {
		autodream_metrics.ConsolidationErrorsTotal.WithLabelValues(mode, "ingestCompletedTasks", "begin_tx_failed").Inc()
		slog.Error("AutoDream: failed to begin tx for completed tasks", "error", txErr)
		return
	}
	defer tx.Rollback(ctx)

	var query string
	if w.pool.IsSQLite() {
		query = "SELECT id, title, COALESCE(payload, '{}') FROM shared_tasks WHERE status = 'COMPLETED' LIMIT 500"
	} else {
		query = "SELECT id, title, COALESCE(payload, '{}') FROM shared_tasks WHERE status = 'COMPLETED' LIMIT 500 FOR UPDATE SKIP LOCKED"
	}

	rows, errQuery := tx.Query(ctx, query)`, 1)

	strContent = strings.Replace(strContent, `	if err != nil {
		slog.Error("AutoDream: failed to fetch completed tasks", "error", err)
		return
	}`, `	if errQuery != nil {
		slog.Error("AutoDream: failed to fetch completed tasks", "error", errQuery)
		return
	}`, 1)

	strContent = strings.Replace(strContent, `	mode := kairos.GetMode()
	start := time.Now()

	defer func() {
		autodream_metrics.BatchProcessingDuration.WithLabelValues(mode, "compressSessionData").Observe(time.Since(start).Seconds())
	}()

	tx, txErr := w.pool.Begin(ctx)
	if txErr != nil {
		autodream_metrics.ConsolidationErrorsTotal.WithLabelValues(mode, "compressSessionData", "begin_tx_failed").Inc()
		slog.Error("AutoDream: failed to begin transaction for compression", "error", txErr)
		return
	}
	defer tx.Rollback(ctx)

	threshold := time.Now().Add(-1 * time.Hour).UTC()
	var query string
	if w.pool.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < ? LIMIT 50"
	} else {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < $1 LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query, threshold)`, `	mode := kairos.GetMode()
	start := time.Now()

	defer func() {
		autodream_metrics.BatchProcessingDuration.WithLabelValues(mode, "compressSessionData").Observe(time.Since(start).Seconds())
	}()

	tx, txErr := w.pool.Begin(ctx)
	if txErr != nil {
		autodream_metrics.ConsolidationErrorsTotal.WithLabelValues(mode, "compressSessionData", "begin_tx_failed").Inc()
		slog.Error("AutoDream: failed to begin transaction for compression", "error", txErr)
		return
	}
	defer tx.Rollback(ctx)

	threshold := time.Now().Add(-1 * time.Hour).UTC()
	var query string
	if w.pool.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < ? LIMIT 50"
	} else {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < $1 LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	rows, errQuery := tx.Query(ctx, query, threshold)`, 1)

	strContent = strings.Replace(strContent, `	if err != nil {
		slog.Error("AutoDream: failed to fetch stale sessions", "error", err)
		return
	}`, `	if errQuery != nil {
		slog.Error("AutoDream: failed to fetch stale sessions", "error", errQuery)
		return
	}`, 1)

	err = ioutil.WriteFile("srcs/server/orchestration/autodream.go", []byte(strContent), 0644)
	if err != nil {
		fmt.Println("Error writing file:", err)
		return
	}
	fmt.Println("File successfully patched")
}
