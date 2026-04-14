package onboarding

import (
	"os"
)

// OnboardingStatus represents the status of the Day One setup
type OnboardingStatus struct {
	IsStandalone bool   `json:"is_standalone"`
	Mode         string `json:"mode"`
	Status       string `json:"status"`
}

// GetStatus returns the current onboarding setup status
func GetStatus() *OnboardingStatus {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	mode := "Cloud-Native K8s"
	if isStandalone {
		mode = "Standalone Desktop"
	}

	return &OnboardingStatus{
		IsStandalone: isStandalone,
		Mode:         mode,
		Status:       "Ready for Day One setup",
	}
}
