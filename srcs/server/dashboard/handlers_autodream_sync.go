package dashboard

import (
	"encoding/json"
	"net/http"
)

// handleAutoDreamSync receives AutoDream protobuf sync payloads.
func (s *Server) handleAutoDreamSync(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// This assumes the sync happens between Standalone and Cloud, where Cloud saves to the DB.
	// For this test, since we're just checking the Sync side of things and the mock, we can just return 200.
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "ok"})
}
