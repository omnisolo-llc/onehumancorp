package analytics

import (
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
)

type Event struct {
	ID        string `json:"id"`
	UserID    string `json:"user_id"`
	EventName string `json:"event_name"`
	Metadata  string `json:"metadata"`
}

type Tracker struct {
	db *sql.DB
}

func NewTracker(db *sql.DB) *Tracker {
	_, err := db.Exec(`
		CREATE TABLE IF NOT EXISTS analytics_events (
			id TEXT PRIMARY KEY,
			user_id TEXT,
			event_name TEXT,
			metadata TEXT
		);
	`)
	if err != nil {
		log.Fatalf("Failed to initialize analytics tables: %v", err)
	}
	return &Tracker{
		db: db,
	}
}

func (t *Tracker) HandleTrackEvent(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req Event
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	_, err := t.db.Exec(`
		INSERT INTO analytics_events (id, user_id, event_name, metadata)
		VALUES ($1, $2, $3, $4)
	`, req.ID, req.UserID, req.EventName, req.Metadata)

	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(req)
}
