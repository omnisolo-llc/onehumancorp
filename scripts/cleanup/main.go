package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"
	"path/filepath"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/orchestration"
)

func main() {
	// 1. Identify DB file
	homeDir, err := os.UserHomeDir()
	if err != nil {
		homeDir = "/tmp"
	}
	dbPath := filepath.Join(homeDir, ".openclaw", "ohc.db")
	if envPath := os.Getenv("OHC_DB_PATH"); envPath != "" {
		dbPath = envPath
	}

	// Make sure dir exists
	os.MkdirAll(filepath.Dir(dbPath), 0755)

	// 2. Open DB
	db, err := orchestration.NewSIPDB(dbPath)
	if err != nil {
		log.Fatalf("Failed to open DB: %v", err)
	}
	defer db.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// 3. Setup cleanup mission record in DB
	agentID := "system_janitor"
	missionID := uuid.New().String()

	err = db.Heartbeat(ctx, agentID, "CLEANER", "RUNNING")
	if err != nil {
		log.Printf("Warning: Failed to log heartbeat: %v", err)
	}

	// Delegate self a cleanup mission
	msg := orchestration.Message{
		ID: missionID,
		Content: "Pruning obsolete data and branches",
		Type: orchestration.EventTask,
	}
	db.DelegateMission(ctx, missionID, "CLEANER", msg)

	// 4. Run database cleanup (pruning completed/stale missions)
	log.Println("[CLEANUP] Pruning stale DB missions...")
	err = db.PruneStaleMissions(ctx, 24 * time.Hour) // Prune missions > 24 hours old
	if err != nil {
		log.Printf("Warning: Failed to prune stale DB missions: %v", err)
	} else {
		log.Println("[CLEANUP] Successfully pruned stale DB missions.")
	}

	// Complete mission
	db.CompleteMission(ctx, missionID)

	// Update shared memory status
	db.UpdateMemory(ctx, "last_stale_prune_time", time.Now().UTC().Format(time.RFC3339))
	db.Heartbeat(ctx, agentID, "CLEANER", "IDLE")

	fmt.Println("Cleanup tasks completed successfully.")
}
