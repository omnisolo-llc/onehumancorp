package onboarding

import (
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

type Diagnostic struct {
	Check   string `json:"check"`
	Status  string `json:"status"`
	Message string `json:"message"`
}

type VerificationResponse struct {
	Status      string       `json:"status"`
	Mode        string       `json:"mode"`
	Diagnostics []Diagnostic `json:"diagnostics"`
}

func VerifyEnvironment(endpoint string) (*VerificationResponse, error) {
	client := &http.Client{Timeout: 5 * time.Second}
	resp, err := client.Get(endpoint + "/api/wizard/onboarding_verify")
	if err != nil {
		return nil, fmt.Errorf("failed to contact backend: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("backend returned status %d", resp.StatusCode)
	}

	var data VerificationResponse
	if err := json.NewDecoder(resp.Body).Decode(&data); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}
	return &data, nil
}
