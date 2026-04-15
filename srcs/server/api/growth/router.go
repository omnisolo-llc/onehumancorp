package growth

import (
	"net/http"
	"github.com/onehumancorp/mono/lib/analytics"
)

func RegisterRoutes(mux *http.ServeMux, tracker *analytics.Tracker) {
	handler := NewGrowthHandler(tracker)
	mux.HandleFunc("/api/growth/visit", handler.HandleTrackVisit)
	mux.HandleFunc("/api/growth/conversion", handler.HandleTrackConversion)
	mux.HandleFunc("/api/growth/invite", handler.HandleInviteTeam)
	mux.HandleFunc("/api/growth/accept", handler.HandleAcceptInvite)
}
