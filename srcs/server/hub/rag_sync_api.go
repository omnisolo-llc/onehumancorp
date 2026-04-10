package hub

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// HandleRAGSync handles incoming RAG sync requests from standalone nodes.
func (h *Hub) HandleRAGSync(w http.ResponseWriter, r *http.Request) {
	// 1. Authenticate (simulate reading SPIFFE ID or user claims)
	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	// 2. Read Payload (enforce max body size)
	r.Body = http.MaxBytesReader(w, r.Body, 10<<20) // 10MB limit
	defer r.Body.Close()

	var records []RAGSyncRecord
	if err := json.NewDecoder(r.Body).Decode(&records); err != nil {
		http.Error(w, "Bad Request", http.StatusBadRequest)
		return
	}

	// 3. Process via RAGSyncService
	if err := h.RAGSyncService.ProcessIncomingSync(r.Context(), records); err != nil {
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
