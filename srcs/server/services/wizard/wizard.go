package wizard

import (
	"context"
	"encoding/json"
	"net/http"
	"sync"
	"strings"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.GetMeterProvider().Meter("ohc_wizard")
	agentConfigCounter metric.Int64Counter
	promptTuningCounter metric.Int64Counter
)

func init() {
    var err error
    agentConfigCounter, err = meter.Int64Counter("ohc_agent_configurations_total")
    if err != nil {
        panic(err)
    }
	promptTuningCounter, err = meter.Int64Counter("ohc_prompt_tuning_total")
	if err != nil {
		panic(err)
	}
}

type AgentConfig struct {
	Role         string `json:"role"`
	Provider     string `json:"provider"`
	Capabilities map[string]bool `json:"capabilities"`
	WorkHours    float64 `json:"work_hours"`
}

func HandleConfigWizard(w http.ResponseWriter, r *http.Request) {
	ctx := context.Background()
	var config AgentConfig
	if err := json.NewDecoder(r.Body).Decode(&config); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	agentConfigCounter.Add(ctx, 1)

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status": "success"}`))
}

type PromptTuningConfig struct {
	Personality string   `json:"personality"`
	DomainFocus []string `json:"domain_focus"`
}

func HandlePromptTuning(w http.ResponseWriter, r *http.Request) {
	ctx := context.Background()
	var config PromptTuningConfig
	if err := json.NewDecoder(r.Body).Decode(&config); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	promptTuningCounter.Add(ctx, 1)

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status": "success"}`))
}

func IsExpertMode(profile map[string]string) bool {
    if v, ok := profile["expert_mode"]; ok && v == "true" {
        return true
    }
    return false
}

var drafts = make(map[string][]byte)
var draftsMutex sync.RWMutex

func extractToken(r *http.Request) string {
	authHeader := r.Header.Get("Authorization")
	if strings.HasPrefix(authHeader, "Bearer ") {
		return strings.TrimPrefix(authHeader, "Bearer ")
	}
	return "anonymous"
}

func HandleSaveDraft(w http.ResponseWriter, r *http.Request) {
	userId := extractToken(r)

	var payload map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	data, err := json.Marshal(payload)
	if err != nil {
		http.Error(w, "Failed to marshal draft", http.StatusInternalServerError)
		return
	}

	draftsMutex.Lock()
	drafts[userId] = data
	draftsMutex.Unlock()

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status": "saved"}`))
}

func HandleGetDraft(w http.ResponseWriter, r *http.Request) {
	userId := extractToken(r)

	draftsMutex.RLock()
	data, ok := drafts[userId]
	draftsMutex.RUnlock()

	w.Header().Set("Content-Type", "application/json")
	if !ok {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{}`))
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write(data)
}
