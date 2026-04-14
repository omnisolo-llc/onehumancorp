import re
with open("srcs/server/dashboard/handlers_growth.go", "r") as f:
    code = f.read()

new_struct = """
type viralBridgeRequest struct {
	Inviter string `json:"inviter"`
	AssetID string `json:"asset_id"`
}
"""

new_func = """
func (s *Server) handleSovereignToCloudInvite(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req viralBridgeRequest
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}
	if req.Inviter == "" || req.AssetID == "" {
		http.Error(w, "inviter and asset_id are required", http.StatusBadRequest)
		return
	}

	// Simulated logging of the bridge event
	s.mu.Lock()
	defer s.mu.Unlock()

	w.WriteHeader(http.StatusAccepted)
	writeJSON(w, map[string]string{"status": "bridge_initiated"})
}
"""

if "type viralBridgeRequest" not in code:
    code += new_struct + new_func

with open("srcs/server/dashboard/handlers_growth.go", "w") as f:
    f.write(code)
