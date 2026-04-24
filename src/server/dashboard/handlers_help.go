package dashboard

import (
	"encoding/json"
	"net/http"
)

type HelpChatRequest struct {
	Query string `json:"query"`
}

type HelpChatResponse struct {
	Reply       string `json:"reply"`
	ArticleLink string `json:"article_link,omitempty"`
}

func (server *Server) handleHelpChat(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req HelpChatRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	// Mock response from AI Help Agent
	resp := HelpChatResponse{
		Reply:       "I can definitely help with that! Here is a summary of how to set up your business online with One Human Corp.",
		ArticleLink: "/help/articles/getting-started",
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}

func (server *Server) handleChangelog(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Mock changelog parsed from RELEASE_NOTES.md
	logs := []map[string]interface{}{
		{
			"version":     "v1.2.0",
			"date":        "April 2026",
			"description": "Introduced the interactive Help Center and AI Help Chat.",
			"features": []string{
				"Searchable Help Center.",
				"AI Help Agent.",
				"Interactive Walkthroughs.",
			},
		},
		{
			"version":     "v1.1.0",
			"date":        "March 2026",
			"description": "Improved AI Agent hiring flow and added new dashboard metrics.",
		},
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(logs)
}

func (server *Server) handleTutorialsVideos(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Mock video metadata
	videos := []map[string]string{
		{"title": "How to set up your store", "url": "https://example.com/vid1"},
		{"title": "Adding your first product", "url": "https://example.com/vid2"},
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(videos)
}
