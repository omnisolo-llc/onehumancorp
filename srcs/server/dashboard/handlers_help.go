package dashboard

import (
	"net/http"
)

type VideoTutorial struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	Duration    string `json:"duration"`
	Description string `json:"description"`
	URL         string `json:"url"`
	Thumbnail   string `json:"thumbnail"`
}

func (s *Server) handleHelpVideos(w http.ResponseWriter, _ *http.Request) {
	videos := []VideoTutorial{
		{
			ID:          "v1",
			Title:       "How to set up your store",
			Duration:    "1:30",
			Description: "A quick guide to adding products, setting prices, and getting your storefront ready.",
			URL:         "https://storage.googleapis.com/ohc-assets/tutorials/setup_store.mp4",
			Thumbnail:   "https://storage.googleapis.com/ohc-assets/tutorials/setup_store_thumb.jpg",
		},
		{
			ID:          "v2",
			Title:       "Accepting your first payment",
			Duration:    "0:45",
			Description: "Learn how to process payments securely using our built-in Stripe integration.",
			URL:         "https://storage.googleapis.com/ohc-assets/tutorials/first_payment.mp4",
			Thumbnail:   "https://storage.googleapis.com/ohc-assets/tutorials/first_payment_thumb.jpg",
		},
		{
			ID:          "v3",
			Title:       "Hiring an AI Agent",
			Duration:    "1:15",
			Description: "See how to add a new AI teammate to handle customer support or marketing.",
			URL:         "https://storage.googleapis.com/ohc-assets/tutorials/hire_agent.mp4",
			Thumbnail:   "https://storage.googleapis.com/ohc-assets/tutorials/hire_agent_thumb.jpg",
		},
	}
	writeJSON(w, videos)
}
