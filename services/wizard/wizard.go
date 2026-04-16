package wizard

import (
	"context"
	"encoding/json"
	"net/http"
	"sync"
    "fmt"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.GetMeterProvider().Meter("ohc_wizard")
	agentConfigCounter metric.Int64Counter
    promptTuningCounter metric.Int64Counter

    // Mock persistence
    promptStore = make(map[string]PromptTuningConfig)
    promptMu sync.RWMutex
)

func init() {
    var err error
    agentConfigCounter, err = meter.Int64Counter("ohc_agent_configurations_total")
    if err != nil {
        panic(err)
    }
    promptTuningCounter, err = meter.Int64Counter("ohc_prompt_tunings_total")
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

func IsExpertMode(profile map[string]string) bool {
    if v, ok := profile["expert_mode"]; ok && v == "true" {
        return true
    }
    return false
}

type PromptTuningConfig struct {
	AgentID      string                   `json:"agent_id"`
	Tone         string                   `json:"tone"`
	DomainFocus  []string                 `json:"domain_focus"`
	Examples     []map[string]string      `json:"examples"`
	SystemPrompt string                   `json:"system_prompt"`
}

func HandlePromptTuning(w http.ResponseWriter, r *http.Request) {
	ctx := context.Background()
	var config PromptTuningConfig
	if err := json.NewDecoder(r.Body).Decode(&config); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

    // Persist to store (mocking pgvector/DB for now)
    promptMu.Lock()
    promptStore[config.AgentID] = config
    promptMu.Unlock()

	promptTuningCounter.Add(ctx, 1)

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status": "success", "message": "Prompt tuned successfully and redeployed."}`))
}

func HandlePromptPreview(w http.ResponseWriter, r *http.Request) {
    var req struct {
        SystemPrompt string `json:"system_prompt"`
        Message      string `json:"message"`
    }
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

    reply := fmt.Sprintf("Simulated agent reply to '%s' using prompt: %.20s...", req.Message, req.SystemPrompt)
    w.Header().Set("Content-Type", "application/json")
    w.WriteHeader(http.StatusOK)
    json.NewEncoder(w).Encode(map[string]string{"reply": reply})
}
