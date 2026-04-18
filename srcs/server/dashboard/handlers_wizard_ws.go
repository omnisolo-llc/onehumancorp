package dashboard

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
)

// modelProviderWizardConfig holds the provider configuration being built by
// the model-provider wizard.  Values match ModelProviderConfig in wizard.proto.
type modelProviderWizardConfig struct {
	ID           string   `json:"id,omitempty"`
	ProviderType int32    `json:"provider_type,omitempty"`
	Name         string   `json:"name,omitempty"`
	BaseURL      string   `json:"base_url,omitempty"`
	APIKey       string   `json:"api_key,omitempty"`
	Model        string   `json:"model,omitempty"`
	Models       []string `json:"models,omitempty"`
	Enabled      bool     `json:"enabled,omitempty"`
	IsOfficial   bool     `json:"is_official,omitempty"`
}

// modelProviderWizardRequest mirrors ModelProviderWizardRequest in wizard.proto.
type modelProviderWizardRequest struct {
	Step      int32                     `json:"step"`
	Provider  modelProviderWizardConfig `json:"provider"`
	AgentType string                    `json:"agent_type,omitempty"`
}

// modelProviderWizardResponse mirrors ModelProviderWizardResponse in wizard.proto.
type modelProviderWizardResponse struct {
	Step               int32                     `json:"step"`
	TotalSteps         int32                     `json:"total_steps"`
	Title              string                    `json:"title"`
	Instruction        string                    `json:"instruction"`
	ValidationErrors   []string                  `json:"validation_errors,omitempty"`
	Complete           bool                      `json:"complete,omitempty"`
	ConfiguredProvider modelProviderWizardConfig `json:"configured_provider,omitempty"`
}

// nlChatRequest mirrors WizardNlChatRequest in wizard.proto.
type nlChatRequest struct {
	SessionID    string                          `json:"session_id,omitempty"`
	Message      string                          `json:"message"`
	PartialState *wizardBootstrapBusinessRequest `json:"partial_state,omitempty"`
}

// nlChatResponse mirrors WizardNlChatResponse in wizard.proto.
type nlChatResponse struct {
	SessionID     string            `json:"session_id,omitempty"`
	Reply         string            `json:"reply"`
	FieldUpdates  map[string]string `json:"field_updates,omitempty"`
	ReadyToSubmit bool              `json:"ready_to_submit,omitempty"`
}

const totalProviderWizardSteps = int32(5)

// handleWizardModelProvider implements the step-by-step model provider
// configuration wizard.  It accepts a ModelProviderWizardRequest and returns
// a ModelProviderWizardResponse, matching the ModelProviderWizard RPC in
// wizard.proto.  Each POST advances one step, validates the current inputs,
// and returns instructions for the next step.
func (s *Server) handleWizardModelProvider(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req modelProviderWizardRequest
	dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20))
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload: "+err.Error(), http.StatusBadRequest)
		return
	}
	writeJSON(w, advanceProviderWizardStep(req))
}

// advanceProviderWizardStep validates the current step and returns the next
// step response for the model provider configuration wizard.
func advanceProviderWizardStep(req modelProviderWizardRequest) modelProviderWizardResponse {
	var errs []string

	switch req.Step {
	case 0:
		return modelProviderWizardResponse{
			Step:        1,
			TotalSteps:  totalProviderWizardSteps,
			Title:       "Select Provider",
			Instruction: "Choose the AI provider you want to configure (e.g. OpenAI, Anthropic, MiniMax, Ollama, Custom).",
		}

	case 1:
		if req.Provider.Name == "" && req.Provider.ProviderType == 0 {
			errs = append(errs, "provider name or provider_type is required")
		}
		if len(errs) > 0 {
			return modelProviderWizardResponse{
				Step:             1,
				TotalSteps:       totalProviderWizardSteps,
				Title:            "Select Provider",
				Instruction:      "Choose the AI provider you want to configure.",
				ValidationErrors: errs,
			}
		}
		baseURL := req.Provider.BaseURL
		if baseURL == "" {
			baseURL = defaultBaseURLForProviderType(req.Provider.ProviderType, req.Provider.Name)
		}
		cfg := req.Provider
		cfg.BaseURL = baseURL
		return modelProviderWizardResponse{
			Step:               2,
			TotalSteps:         totalProviderWizardSteps,
			Title:              "Base URL",
			Instruction:        fmt.Sprintf("Enter the API base URL for %s (default: %s).", providerDisplayNameForWizard(req.Provider), baseURL),
			ConfiguredProvider: cfg,
		}

	case 2:
		if strings.TrimSpace(req.Provider.BaseURL) == "" {
			errs = append(errs, "base_url is required")
		} else if !strings.HasPrefix(req.Provider.BaseURL, "http") {
			errs = append(errs, "base_url must start with http:// or https://")
		}
		if len(errs) > 0 {
			return modelProviderWizardResponse{
				Step:             2,
				TotalSteps:       totalProviderWizardSteps,
				Title:            "Base URL",
				Instruction:      "Enter the API base URL.",
				ValidationErrors: errs,
				ConfiguredProvider: req.Provider,
			}
		}
		return modelProviderWizardResponse{
			Step:               3,
			TotalSteps:         totalProviderWizardSteps,
			Title:              "API Key",
			Instruction:        "Enter your API key.  Leave empty for local providers that do not require authentication (e.g. Ollama).",
			ConfiguredProvider: req.Provider,
		}

	case 3:
		return modelProviderWizardResponse{
			Step:               4,
			TotalSteps:         totalProviderWizardSteps,
			Title:              "Select Model",
			Instruction:        "Choose the primary model to use.  You can add additional models later.",
			ConfiguredProvider: req.Provider,
		}

	case 4:
		if strings.TrimSpace(req.Provider.Model) == "" && len(req.Provider.Models) == 0 {
			errs = append(errs, "at least one model is required")
		}
		if len(errs) > 0 {
			return modelProviderWizardResponse{
				Step:             4,
				TotalSteps:       totalProviderWizardSteps,
				Title:            "Select Model",
				Instruction:      "Enter the model name.",
				ValidationErrors: errs,
				ConfiguredProvider: req.Provider,
			}
		}
		return modelProviderWizardResponse{
			Step:               5,
			TotalSteps:         totalProviderWizardSteps,
			Title:              "Review & Save",
			Instruction:        "Review your configuration and tap Save to add this provider.",
			ConfiguredProvider: req.Provider,
		}

	case 5:
		return modelProviderWizardResponse{
			Step:               5,
			TotalSteps:         totalProviderWizardSteps,
			Title:              "Review & Save",
			Instruction:        "Configuration complete.",
			Complete:           true,
			ConfiguredProvider: req.Provider,
		}

	default:
		return modelProviderWizardResponse{
			Step:             req.Step,
			TotalSteps:       totalProviderWizardSteps,
			Title:            "Unknown Step",
			Instruction:      "Unrecognised wizard step.",
			ValidationErrors: []string{"unknown step"},
		}
	}
}

// handleWizardNlChat handles the NL wizard chat endpoint, matching the NlChat
// RPC in wizard.proto.  It parses a natural-language message and returns a
// reply plus any field values it was able to extract.
func (s *Server) handleWizardNlChat(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req nlChatRequest
	dec := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20))
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload: "+err.Error(), http.StatusBadRequest)
		return
	}

	msg := strings.TrimSpace(req.Message)
	if msg == "" {
		http.Error(w, "message is required", http.StatusBadRequest)
		return
	}

	fieldUpdates := extractFieldsFromNLMessage(msg, req.PartialState)
	reply, readyToSubmit := buildNLReply(msg, fieldUpdates, req.PartialState)

	sessionID := req.SessionID
	if sessionID == "" {
		sessionID = fmt.Sprintf("nl-%d", len(msg))
	}

	writeJSON(w, nlChatResponse{
		SessionID:     sessionID,
		Reply:         reply,
		FieldUpdates:  fieldUpdates,
		ReadyToSubmit: readyToSubmit,
	})
}

// extractFieldsFromNLMessage extracts structured field values from a
// natural-language message using simple heuristics.
func extractFieldsFromNLMessage(msg string, partial *wizardBootstrapBusinessRequest) map[string]string {
	lower := strings.ToLower(msg)
	updates := make(map[string]string)

	// Company name: "company is X", "called X", "named X", etc.
	for _, prefix := range []string{"company is ", "called ", "named ", "company name is ", "name is "} {
		if idx := strings.Index(lower, prefix); idx >= 0 {
			rest := strings.TrimSpace(msg[idx+len(prefix):])
			if rest != "" {
				name := strings.SplitN(rest, " ", 2)[0]
				name = strings.Trim(name, ".,;:\"'")
				if name != "" && len(name) < 80 {
					updates["company_name"] = name
					break
				}
			}
		}
	}

	// Industry detection.
	industries := map[string]string{
		"real estate": "Real Estate", "software": "Software", "tech": "Technology",
		"marketing": "Marketing", "finance": "Finance", "healthcare": "Healthcare",
		"education": "Education", "retail": "Retail", "legal": "Legal",
		"accounting": "Finance", "restaurant": "Food & Beverage",
	}
	for keyword, industry := range industries {
		if strings.Contains(lower, keyword) {
			updates["industry"] = industry
			break
		}
	}

	// Goals detection.
	goalKeywords := map[string]string{
		"support": "Support", "build software": "Build software",
		"marketing": "Marketing", "data": "Data",
	}
	for keyword, goal := range goalKeywords {
		if strings.Contains(lower, keyword) {
			updates["goals"] = goal
			break
		}
	}

	// Size detection.
	switch {
	case strings.Contains(lower, "small") || strings.Contains(lower, "startup"):
		updates["size"] = "S"
	case strings.Contains(lower, "medium") || strings.Contains(lower, "mid-size"):
		updates["size"] = "M"
	case strings.Contains(lower, "large") || strings.Contains(lower, "enterprise"):
		updates["size"] = "L"
	}

	// Admin name.
	for _, prefix := range []string{"admin name is ", "my name is ", "i am "} {
		if idx := strings.Index(lower, prefix); idx >= 0 {
			rest := strings.TrimSpace(msg[idx+len(prefix):])
			if name := strings.SplitN(rest, ",", 2)[0]; name != "" && len(name) < 80 {
				updates["admin_name"] = strings.TrimSpace(name)
				break
			}
		}
	}

	// Admin email.
	for _, prefix := range []string{"email is ", "my email is ", "email: "} {
		if idx := strings.Index(lower, prefix); idx >= 0 {
			rest := strings.TrimSpace(msg[idx+len(prefix):])
			if email := strings.SplitN(rest, " ", 2)[0]; strings.Contains(email, "@") {
				updates["admin_email"] = strings.Trim(email, ".,;")
				break
			}
		}
	}

	return updates
}

// buildNLReply constructs a helpful assistant reply and determines whether the
// form has enough data to submit.
func buildNLReply(msg string, updates map[string]string, partial *wizardBootstrapBusinessRequest) (string, bool) {
	if len(updates) == 0 {
		return "Got it! Could you tell me more about your company? For example: company name, industry, size, and your admin email.", false
	}

	var parts []string
	if name, ok := updates["company_name"]; ok {
		parts = append(parts, fmt.Sprintf("company name set to %q", name))
	}
	if industry, ok := updates["industry"]; ok {
		parts = append(parts, fmt.Sprintf("industry set to %q", industry))
	}
	if size, ok := updates["size"]; ok {
		sizeLabel := map[string]string{"S": "Small", "M": "Medium", "L": "Large"}[size]
		parts = append(parts, fmt.Sprintf("size set to %s", sizeLabel))
	}
	if goal, ok := updates["goals"]; ok {
		parts = append(parts, fmt.Sprintf("goal %q added", goal))
	}

	reply := "Great! I've " + strings.Join(parts, ", ") + "."

	// Determine readiness: require company name and admin email at minimum.
	companyName := updates["company_name"]
	adminEmail := updates["admin_email"]
	if partial != nil {
		if companyName == "" {
			companyName = partial.CompanyName
		}
		if adminEmail == "" {
			adminEmail = partial.AdminEmail
		}
	}

	readyToSubmit := companyName != "" && adminEmail != ""
	if !readyToSubmit {
		var missing []string
		if companyName == "" {
			missing = append(missing, "company name")
		}
		if adminEmail == "" {
			missing = append(missing, "admin email")
		}
		reply += " Still need: " + strings.Join(missing, ", ") + "."
	} else {
		reply += " Looks like we have everything needed — ready to launch!"
	}

	return reply, readyToSubmit
}

// defaultBaseURLForProviderType returns a sensible default base URL for a
// given ProviderType value.  Values match model.proto's ProviderType enum.
func defaultBaseURLForProviderType(pt int32, name string) string {
	switch pt {
	case 1: // PROVIDER_TYPE_OPENAI
		return "https://api.openai.com/v1"
	case 2: // PROVIDER_TYPE_ANTHROPIC
		return "https://api.anthropic.com/v1"
	case 3: // PROVIDER_TYPE_GOOGLE
		return "https://generativelanguage.googleapis.com/v1beta"
	case 4: // PROVIDER_TYPE_GROQ
		return "https://api.groq.com/openai/v1"
	case 5: // PROVIDER_TYPE_OLLAMA
		return "http://localhost:11434/v1"
	case 6: // PROVIDER_TYPE_OPENROUTER
		return "https://openrouter.ai/api/v1"
	case 8: // PROVIDER_TYPE_AZURE
		return "https://YOUR_RESOURCE.openai.azure.com/openai/deployments/YOUR_DEPLOYMENT"
	case 10: // PROVIDER_TYPE_MINIMAX
		return "https://api.minimax.io/v1"
	}
	lower := strings.ToLower(strings.TrimSpace(name))
	switch {
	case strings.Contains(lower, "openai"):
		return "https://api.openai.com/v1"
	case strings.Contains(lower, "anthropic"):
		return "https://api.anthropic.com/v1"
	case strings.Contains(lower, "ollama"):
		return "http://localhost:11434/v1"
	case strings.Contains(lower, "minimax"):
		return "https://api.minimax.io/v1"
	}
	return ""
}

// providerDisplayNameForWizard returns a human-readable name for the provider.
func providerDisplayNameForWizard(p modelProviderWizardConfig) string {
	if p.Name != "" {
		return p.Name
	}
	switch p.ProviderType {
	case 1:
		return "OpenAI"
	case 2:
		return "Anthropic"
	case 3:
		return "Google"
	case 4:
		return "Groq"
	case 5:
		return "Ollama"
	case 10:
		return "MiniMax"
	default:
		return "Custom"
	}
}
