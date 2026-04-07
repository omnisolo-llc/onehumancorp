package api

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type RAGSyncHandler struct {
	service hub.RAGSyncService
}

func NewRAGSyncHandler(service hub.RAGSyncService) *RAGSyncHandler {
	return &RAGSyncHandler{
		service: service,
	}
}

func (h *RAGSyncHandler) HandleIncomingSync(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var records []hub.RAGSyncRecord
	if err := json.NewDecoder(r.Body).Decode(&records); err != nil {
		http.Error(w, "Invalid payload", http.StatusBadRequest)
		return
	}

	if err := h.service.ProcessIncomingSync(r.Context(), records); err != nil {
		http.Error(w, "Failed to process sync", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
