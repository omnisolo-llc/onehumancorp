package dashboard

import (
	"encoding/json"
	"net/http"
)

type HelpArticle struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	Category    string `json:"category"`
	Description string `json:"description"`
	Content     string `json:"content"`
}

var defaultHelpArticles = []HelpArticle{
	{
		ID:          "1",
		Title:       "Getting Started with OneHumanCorp",
		Category:    "Getting Started",
		Description: "Learn the basics of setting up your business profile.",
		Content:     "Welcome to OneHumanCorp! To get started, navigate to your Settings and fill out your Business Name and Description.",
	},
	{
		ID:          "2",
		Title:       "How to accept payments",
		Category:    "Payments",
		Description: "Configure your payment settings to start earning.",
		Content:     "Go to the Payments tab to connect your Stripe account. Once connected, you can toggle online and in-person payments.",
	},
	{
		ID:          "3",
		Title:       "Hiring your first AI Agent",
		Category:    "AI Agents",
		Description: "Learn how to delegate tasks to your AI workforce.",
		Content:     "Click the Agents tab, select 'Hire Agent', and choose a role like 'Customer Support' or 'Marketing'.",
	},
}

func (s *Server) handleHelpArticles(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(defaultHelpArticles); err != nil {
		http.Error(w, "Failed to encode response", http.StatusInternalServerError)
	}
}

func (s *Server) handleHelpSearch(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	query := r.URL.Query().Get("q")
	if query == "" {
		s.handleHelpArticles(w, r)
		return
	}

	// Simple mock search
	var results []HelpArticle
	for _, article := range defaultHelpArticles {
		if article.Category == query {
			results = append(results, article)
		}
	}

	if len(results) == 0 {
		results = defaultHelpArticles // Fallback for testing
	}

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(results); err != nil {
		http.Error(w, "Failed to encode response", http.StatusInternalServerError)
	}
}
