package dashboard

import (
	"encoding/json"
	"net/http"
)

type VideoTutorial struct {
	ID           string `json:"id"`
	Title        string `json:"title"`
	Duration     string `json:"duration"`
	ThumbnailURL string `json:"thumbnail_url"`
	VideoURL     string `json:"video_url"`
	Description  string `json:"description"`
}

func HandleGetVideoTutorials(w http.ResponseWriter, r *http.Request) {
	videos := []VideoTutorial{
		{ID: "1", Title: "How to add your first product", Duration: "1:20", Description: "Learn the basics of setting up your storefront and adding a new product.", VideoURL: "https://example.com/video1", ThumbnailURL: "https://example.com/thumb1"},
		{ID: "2", Title: "Setting up automated support", Duration: "0:55", Description: "Configure your AI team to answer customer questions automatically.", VideoURL: "https://example.com/video2", ThumbnailURL: "https://example.com/thumb2"},
		{ID: "3", Title: "Accepting your first payment", Duration: "1:15", Description: "Connect Stripe and process your first transaction.", VideoURL: "https://example.com/video3", ThumbnailURL: "https://example.com/thumb3"},
		{ID: "4", Title: "Managing your inventory", Duration: "0:45", Description: "Track stock levels and set up low-stock alerts.", VideoURL: "https://example.com/video4", ThumbnailURL: "https://example.com/thumb4"},
		{ID: "5", Title: "Understanding your dashboard", Duration: "1:25", Description: "A walkthrough of your main dashboard metrics.", VideoURL: "https://example.com/video5", ThumbnailURL: "https://example.com/thumb5"},
		{ID: "6", Title: "Inviting team members", Duration: "0:30", Description: "How to add staff and assign roles.", VideoURL: "https://example.com/video6", ThumbnailURL: "https://example.com/thumb6"},
		{ID: "7", Title: "Customizing your store theme", Duration: "1:10", Description: "Change colors, fonts, and layout options.", VideoURL: "https://example.com/video7", ThumbnailURL: "https://example.com/thumb7"},
		{ID: "8", Title: "Setting up custom domains", Duration: "1:00", Description: "Link your own domain name to your store.", VideoURL: "https://example.com/video8", ThumbnailURL: "https://example.com/thumb8"},
		{ID: "9", Title: "Viewing customer orders", Duration: "0:40", Description: "How to find and fulfill orders.", VideoURL: "https://example.com/video9", ThumbnailURL: "https://example.com/thumb9"},
		{ID: "10", Title: "Exporting financial reports", Duration: "0:50", Description: "Download CSV reports for accounting.", VideoURL: "https://example.com/video10", ThumbnailURL: "https://example.com/thumb10"},
	}

	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Access-Control-Allow-Origin", "*")
	json.NewEncoder(w).Encode(videos)
}
