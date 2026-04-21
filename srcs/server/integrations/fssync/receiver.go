package fssync

import (
	"encoding/json"
	"io"
	"net/http"
	"sync"
)

// SyncPayload represents the JSON payload received from the SyncDaemon Uploader
type SyncPayload struct {
	Metadata map[string]string `json:"metadata"`
	Chunks   [][]byte          `json:"chunks"`
}

// Receiver handles incoming chunked file sync requests
type Receiver struct {
	// In a real application, this would interface with Cloud Storage (Postgres/S3)
	mu             sync.Mutex
	ReceivedChunks map[string][][]byte
}

func NewReceiver() *Receiver {
	return &Receiver{
		ReceivedChunks: make(map[string][][]byte),
	}
}

// HandleSync is the HTTP handler for POST /api/hybrid/sync/fs
func (r *Receiver) HandleSync(w http.ResponseWriter, req *http.Request) {
	if req.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	body, err := io.ReadAll(req.Body)
	if err != nil {
		http.Error(w, "Failed to read body", http.StatusBadRequest)
		return
	}
	defer req.Body.Close()

	var payload SyncPayload
	if err := json.Unmarshal(body, &payload); err != nil {
		http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
		return
	}

	filePath, ok := payload.Metadata["filepath"]
	if !ok || filePath == "" {
		http.Error(w, "Missing filepath in metadata", http.StatusBadRequest)
		return
	}

	// Store chunks in memory (for mock/test purposes)
	r.mu.Lock()
	r.ReceivedChunks[filePath] = append(r.ReceivedChunks[filePath], payload.Chunks...)
	r.mu.Unlock()

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}
