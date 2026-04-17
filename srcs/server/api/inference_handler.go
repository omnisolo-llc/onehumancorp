package api

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/integrations/mcp_inference_router"
)

type InferenceHandler struct {
	Router *mcp_inference_router.InferenceRouter
}

func NewInferenceHandler(router *mcp_inference_router.InferenceRouter) *InferenceHandler {
	return &InferenceHandler{Router: router}
}

func (h *InferenceHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req mcp_inference_router.InferenceRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}
	defer r.Body.Close()

	resp, err := h.Router.RouteInference(r.Context(), req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}