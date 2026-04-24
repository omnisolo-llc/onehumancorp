package api

import (
	"encoding/json"
	"net/http"
)

type VideoTutorial struct {
	ID              string `json:"id"`
	Title           string `json:"title"`
	Description     string `json:"description"`
	VideoUrl        string `json:"videoUrl"`
	DurationSeconds int    `json:"durationSeconds"`
}

type HelpHandler struct {
}

func NewHelpHandler() *HelpHandler {
	return &HelpHandler{}
}

func (h *HelpHandler) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/v1/help/video-tutorials", h.handleGetVideoTutorials)
}

func (h *HelpHandler) handleGetVideoTutorials(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	tutorials := []VideoTutorial{
		{ID: "1", Title: "How to set up your store", Description: "Configure your store for the first time.", VideoUrl: "https://example.com/video1.mp4", DurationSeconds: 85},
		{ID: "2", Title: "Accept your first payment", Description: "Link Stripe and process a payment.", VideoUrl: "https://example.com/video2.mp4", DurationSeconds: 60},
		{ID: "3", Title: "Activate your AI Support Agent", Description: "Let AI handle customer support 24/7.", VideoUrl: "https://example.com/video3.mp4", DurationSeconds: 75},
		{ID: "4", Title: "Add a new product", Description: "Create a physical or digital product.", VideoUrl: "https://example.com/video4.mp4", DurationSeconds: 50},
		{ID: "5", Title: "Manage your inventory", Description: "Keep track of stock levels easily.", VideoUrl: "https://example.com/video5.mp4", DurationSeconds: 45},
		{ID: "6", Title: "Customize your storefront", Description: "Change colors, fonts, and layout.", VideoUrl: "https://example.com/video6.mp4", DurationSeconds: 90},
		{ID: "7", Title: "Create a discount code", Description: "Run a sale and share promo codes.", VideoUrl: "https://example.com/video7.mp4", DurationSeconds: 40},
		{ID: "8", Title: "View business analytics", Description: "Understand your sales and traffic.", VideoUrl: "https://example.com/video8.mp4", DurationSeconds: 65},
		{ID: "9", Title: "Set up a custom domain", Description: "Connect your own web address.", VideoUrl: "https://example.com/video9.mp4", DurationSeconds: 80},
		{ID: "10", Title: "Manage user access", Description: "Invite team members to help out.", VideoUrl: "https://example.com/video10.mp4", DurationSeconds: 55},
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(tutorials)
}
