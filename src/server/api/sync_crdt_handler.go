package api

import (
	"database/sql"
	"encoding/json"
	"net/http"
)

type SyncCRDTHandler struct {
	db *sql.DB
}

func NewSyncCRDTHandler(db *sql.DB) *SyncCRDTHandler {
	return &SyncCRDTHandler{db: db}
}

type CRDTDelta struct {
	ID        string `json:"id"`
	EntityID  string `json:"entity_id"`
	Data      string `json:"data"`
	UpdatedAt string `json:"updated_at"`
}

type CRDTPayload struct {
	Deltas []CRDTDelta `json:"deltas"`
}

func (h *SyncCRDTHandler) HandlePost(w http.ResponseWriter, r *http.Request) {
	var payload CRDTPayload
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "Invalid payload", http.StatusBadRequest)
		return
	}

	tx, err := h.db.Begin()
	if err != nil {
		http.Error(w, "Failed to begin transaction", http.StatusInternalServerError)
		return
	}
	defer tx.Rollback()

	stmt, err := tx.Prepare(`
		INSERT INTO crdt_deltas (id, entity_id, data, updated_at)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT (id) DO UPDATE SET
			data = EXCLUDED.data,
			updated_at = EXCLUDED.updated_at
		WHERE crdt_deltas.updated_at < EXCLUDED.updated_at
	`)
	if err != nil {
		http.Error(w, "Failed to prepare statement", http.StatusInternalServerError)
		return
	}
	defer stmt.Close()

	for _, delta := range payload.Deltas {
		_, err := stmt.Exec(delta.ID, delta.EntityID, delta.Data, delta.UpdatedAt)
		if err != nil {
			http.Error(w, "Failed to insert delta", http.StatusInternalServerError)
			return
		}
	}

	if err := tx.Commit(); err != nil {
		http.Error(w, "Failed to commit transaction", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
