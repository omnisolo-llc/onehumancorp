// HandleHandshake validates thin client credentials/OAuth tokens and returns a signed JWT alongside tenant isolation context. GET/POST /api/auth/handshake {"token":"...","tenant_id":"..."}
func (h *Handlers) HandleHandshake(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		Token    string `json:"token"`
		TenantID string `json:"tenant_id"`
		Mode     string `json:"mode"`
	}

	dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20))
	dec.DisallowUnknownFields()
	if err := dec.Decode(&req); err != nil {
		jsonError(w, "invalid JSON", http.StatusBadRequest)
		return
	}

	claims, err := ValidateOIDCToken(req.Token, h.oidcCfg)
	if err != nil {
		jsonError(w, "invalid token or unauthorized", http.StatusUnauthorized)
		return
	}

	user, ok := h.store.GetUser(claims.Subject, req.TenantID)
	if !ok {
		jsonError(w, "user not found for tenant", http.StatusUnauthorized)
		return
	}

	token, err := h.store.IssueToken(user)
	if err != nil {
		jsonError(w, "failed to issue token", http.StatusInternalServerError)
		return
	}
	writeJSON(w, http.StatusOK, map[string]interface{}{"token": token, "tenant_id": req.TenantID, "mode": req.Mode})
}
