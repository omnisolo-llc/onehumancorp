package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
	_ "github.com/jackc/pgx/v5/stdlib"
)

func main() {
	log.SetFlags(0)

	dbPath := os.Getenv("DATABASE_URL")
	if dbPath == "" {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			log.Fatalf("Could not get user home dir: %v", err)
		}
		dbPath = fmt.Sprintf("sqlite://%s/.ohc-local-data/standalone.db", homeDir)
		os.Setenv("DATABASE_URL", dbPath)
	}

	log.Printf("Connecting to DB: %s\n", dbPath)

	ctx := context.Background()

    // Using db.New uses the DATABASE_URL env var and runs migrations if not in memory
	database, err := db.New(ctx)
	if err != nil {
		log.Fatalf("Failed to init DB: %v", err)
	}
	defer database.Close()

    // Ensure basic tables exist in case migrations failed or this is standalone without full migrations
    provider := database.Provider

    _, _ = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS agent_missions (
        id         TEXT PRIMARY KEY,
        status     TEXT NOT NULL,
        payload    TEXT NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        organization_id TEXT DEFAULT 'system'
    );`)

    _, _ = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS meeting_rooms (
        id           TEXT PRIMARY KEY,
        agenda       TEXT NOT NULL DEFAULT '',
        participants TEXT NOT NULL DEFAULT '{}'
    );`)

	// Seed Agent Missions
	seedMissions(ctx, provider)

	// Seed Meeting Rooms
	seedMeetingRooms(ctx, provider)

	log.Println("Mock Data seeded successfully into Database!")
}

func seedMissions(ctx context.Context, provider db.Provider) {
	missions := []struct{
		ID string
		Status string
		Payload map[string]string
	}{
		{
			ID: "mission-setup-1",
			Status: "DONE",
			Payload: map[string]string{"title": "Verify local SQLite database", "description": "Ensure standalone mode connects successfully."},
		},
		{
			ID: "mission-setup-2",
			Status: "PENDING",
			Payload: map[string]string{"title": "Analyze telemetry data", "description": "Check if prometheus is storing metrics."},
		},
		{
			ID: "mission-setup-3",
			Status: "IN_PROGRESS",
			Payload: map[string]string{"title": "Sync with Cloud", "description": "Upload completed missions to OHC cloud."},
		},
	}

	for _, m := range missions {
		b, _ := json.Marshal(m.Payload)
		_, err := provider.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ($1, $2, $3, CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET status=EXCLUDED.status, payload=EXCLUDED.payload", m.ID, m.Status, string(b))
		if err != nil {
			log.Printf("Warning: failed to seed mission %s: %v", m.ID, err)
		}
	}
}

func seedMeetingRooms(ctx context.Context, provider db.Provider) {
	rooms := []struct{
		ID string
		Agenda string
		Participants string
	}{
		{
			ID: "room-daily-standup",
			Agenda: "Daily sync for Standalone environment",
			Participants: `["swe-1", "qa-1"]`,
		},
		{
			ID: "room-incident-response",
			Agenda: "Launch Readiness Check",
			Participants: `["sec-1", "pm-1", "CEO"]`,
		},
	}

	for _, r := range rooms {
		_, err := provider.Exec(ctx, "INSERT INTO meeting_rooms (id, agenda, participants) VALUES ($1, $2, $3) ON CONFLICT(id) DO UPDATE SET agenda=EXCLUDED.agenda, participants=EXCLUDED.participants", r.ID, r.Agenda, r.Participants)
		if err != nil {
			log.Printf("Warning: failed to seed meeting room %s: %v", r.ID, err)
		}
	}
}
