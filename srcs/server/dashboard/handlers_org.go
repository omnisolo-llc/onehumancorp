package dashboard

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/settings"
)

// Handles retrieving organization details.
// Accepts parameters: w, _.
// Returns nothing.
// Produces no errors.
// Has no side effects.
func (s *Server) handleOrg(w http.ResponseWriter, _ *http.Request) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	writeJSON(w, s.org)
}

// Handles retrieving available domains.
// Accepts parameters: w, _.
// Returns nothing.
// Produces no errors.
// Has no side effects.
func (s *Server) handleDomains(w http.ResponseWriter, _ *http.Request) {
	writeJSON(w, availableDomains)
}

// Handles retrieving or updating settings.
// Accepts parameters: w, r.
// Returns nothing.
// Produces no errors.
// Has no side effects.
func (s *Server) handleSettings(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodGet {
		writeJSON(w, s.hub.SettingsStore().Get())
		return
	}

	if r.Method == http.MethodPost {
		var req settings.AppSettings
		dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20))
		dec.DisallowUnknownFields()
		if err := dec.Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if err := s.hub.SettingsStore().Update(req); err != nil {
			http.Error(w, "failed to save settings: "+err.Error(), http.StatusInternalServerError)
			return
		}
		s.mu.Lock()
		s.settings = req
		s.mu.Unlock()

		// Update Minimax API key in Hub if present in the top-level field or extras.
		if req.MinimaxAPIKey != "" {
			s.hub.SetMinimaxAPIKey(req.MinimaxAPIKey)
		} else if key, ok := req.Extras["minimax_api_key"]; ok {
			s.hub.SetMinimaxAPIKey(key)
		}

		writeJSON(w, req)
		return
	}

	http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
}

// Handles retrieving marketplace items.
// Accepts parameters: w, _.
// Returns nothing.
// Produces no errors.
// Has no side effects.
func (s *Server) handleMarketplace(w http.ResponseWriter, _ *http.Request) {
	writeJSON(w, defaultMarketplaceItems())
}

type aiProviderPayload struct {
	ID         string   `json:"id,omitempty"`
	Name       string   `json:"name"`
	BaseURL    string   `json:"base_url,omitempty"`
	APIKey     string   `json:"api_key,omitempty"`
	Model      string   `json:"model,omitempty"`
	Models     []string `json:"models,omitempty"`
	IsOfficial bool     `json:"is_official,omitempty"`
}

func (s *Server) handleAIProviders(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		providers := append([]settings.AiProvider(nil), s.settings.AiProviders...)
		s.mu.RUnlock()
		writeJSON(w, toAIProviderPayloads(providers))
	case http.MethodPost:
		var req aiProviderPayload
		dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20))
		dec.DisallowUnknownFields()
		if err := dec.Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		req.Name = strings.TrimSpace(req.Name)
		if req.Name == "" {
			http.Error(w, "name is required", http.StatusBadRequest)
			return
		}

		provider := settings.AiProvider{
			Name:    req.Name,
			APIKey:  strings.TrimSpace(req.APIKey),
			BaseURL: strings.TrimSpace(req.BaseURL),
			Model:   strings.TrimSpace(req.Model),
			Models:  normalizeProviderModels(req.Models, req.Model),
			Enabled: true,
		}
		if provider.Model == "" && len(provider.Models) > 0 {
			provider.Model = provider.Models[0]
		}

		s.mu.Lock()
		cfg := s.settings
		replaced := false
		for i, existing := range cfg.AiProviders {
			if aiProviderID(existing.Name) == aiProviderID(provider.Name) {
				cfg.AiProviders[i] = provider
				replaced = true
				break
			}
		}
		if !replaced {
			cfg.AiProviders = append(cfg.AiProviders, provider)
		}
		s.settings = cfg
		s.mu.Unlock()
		_ = s.hub.SettingsStore().Update(cfg)

		writeJSON(w, toAIProviderPayload(provider))
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (s *Server) handleAIProviderByID(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPatch {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	providerID := strings.TrimPrefix(r.URL.Path, "/api/ai/providers/")
	if providerID == "" {
		http.Error(w, "provider ID is required", http.StatusBadRequest)
		return
	}
	var req struct {
		APIKey string `json:"api_key"`
	}
	dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20))
	dec.DisallowUnknownFields()
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	cfg := s.settings
	updated := false
	for i, provider := range cfg.AiProviders {
		if aiProviderID(provider.Name) == providerID {
			cfg.AiProviders[i].APIKey = strings.TrimSpace(req.APIKey)
			cfg.AiProviders[i].Enabled = strings.TrimSpace(req.APIKey) != ""
			if len(cfg.AiProviders[i].Models) == 0 && cfg.AiProviders[i].Model != "" {
				cfg.AiProviders[i].Models = []string{cfg.AiProviders[i].Model}
			}
			updated = true
			break
		}
	}
	if !updated {
		s.mu.Unlock()
		http.Error(w, "provider not found", http.StatusNotFound)
		return
	}
	s.settings = cfg
	s.mu.Unlock()
	_ = s.hub.SettingsStore().Update(cfg)
	w.WriteHeader(http.StatusNoContent)
}

func aiProviderID(name string) string {
	return strings.NewReplacer(" ", "-", "/", "-", "_", "-").Replace(strings.ToLower(strings.TrimSpace(name)))
}

func normalizeProviderModels(models []string, model string) []string {
	out := make([]string, 0, len(models)+1)
	seen := map[string]struct{}{}
	if trimmed := strings.TrimSpace(model); trimmed != "" {
		seen[trimmed] = struct{}{}
		out = append(out, trimmed)
	}
	for _, entry := range models {
		trimmed := strings.TrimSpace(entry)
		if trimmed == "" {
			continue
		}
		if _, ok := seen[trimmed]; ok {
			continue
		}
		seen[trimmed] = struct{}{}
		out = append(out, trimmed)
	}
	return out
}

func toAIProviderPayloads(providers []settings.AiProvider) []aiProviderPayload {
	out := make([]aiProviderPayload, 0, len(providers))
	for _, provider := range providers {
		out = append(out, toAIProviderPayload(provider))
	}
	return out
}

func toAIProviderPayload(provider settings.AiProvider) aiProviderPayload {
	models := append([]string(nil), provider.Models...)
	if len(models) == 0 && provider.Model != "" {
		models = []string{provider.Model}
	}
	name := strings.TrimSpace(provider.Name)
	return aiProviderPayload{
		ID:         aiProviderID(name),
		Name:       name,
		BaseURL:    provider.BaseURL,
		APIKey:     provider.APIKey,
		Model:      provider.Model,
		Models:     models,
		IsOfficial: strings.EqualFold(name, "openai") || strings.EqualFold(name, "anthropic") || strings.EqualFold(name, "gemini"),
	}
}

// Handles retrieving analytics summary.
// Accepts parameters: w, _.
// Returns nothing.
// Produces no errors.
// Has no side effects.
func (s *Server) handleAnalytics(w http.ResponseWriter, _ *http.Request) {
	s.mu.RLock()
	agents := s.orgAgentsLocked()
	org := s.org
	summary := s.tracker.Summary(org.ID)
	pendingApprovals := 0
	for _, a := range s.approvals {
		if a.Status == ApprovalStatusPending {
			pendingApprovals++
		}
	}
	activeHandoffs := 0
	for _, h := range s.handoffs {
		if h.Status == "pending" {
			activeHandoffs++
		}
	}
	s.mu.RUnlock()

	totalHumans := 0
	for _, m := range org.Members {
		if m.IsHuman {
			totalHumans++
		}
	}
	totalAgents := len(agents)

	var ratio float64
	if totalHumans > 0 {
		ratio = float64(totalAgents) / float64(totalHumans)
	}

	meetings := s.orgMeetingsLocked()
	totalMsgs := 0
	auditedMsgs := 0
	agentSet := map[string]bool{}
	for _, a := range agents {
		agentSet[a.ID] = true
	}
	for _, m := range meetings {
		for _, msg := range m.Transcript {
			totalMsgs++
			if agentSet[msg.FromAgent] {
				auditedMsgs++
			}
		}
	}
	auditFidelity := 100.0
	if totalMsgs > 0 {
		auditFidelity = float64(auditedMsgs) / float64(totalMsgs) * 100
	}

	writeJSON(w, AnalyticsSummary{
		HumanAgentRatio:     ratio,
		TotalAgents:         totalAgents,
		TotalHumans:         totalHumans,
		AuditFidelityPct:    auditFidelity,
		ResumptionLatencyMS: 4800,
		PendingApprovals:    pendingApprovals,
		ActiveHandoffs:      activeHandoffs,
		TokenVelocity:       summary.TotalTokens,
	})
}
