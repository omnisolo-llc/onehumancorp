package dashboard

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/billing/stripe"
)

func (s *Server) handleBillingPlans(w http.ResponseWriter, r *http.Request) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	writeJSON(w, stripe.AvailablePlans)
}

type CheckoutRequest struct {
	PlanID string `json:"planId"`
}

type CheckoutResponse struct {
	URL string `json:"url"`
}

func (s *Server) handleBillingCheckout(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req CheckoutRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	s.mu.RLock()
	orgID := s.org.ID
	s.mu.RUnlock()

	stripeService := stripe.NewService()
	url, err := stripeService.CreateCheckoutSession(r.Context(), orgID, req.PlanID)
	if err != nil {
		http.Error(w, "failed to create checkout session", http.StatusInternalServerError)
		return
	}

	writeJSON(w, CheckoutResponse{URL: url})
}

func (s *Server) handleBillingPortal(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	s.mu.RLock()
	orgID := s.org.ID
	s.mu.RUnlock()

	stripeService := stripe.NewService()
	url, err := stripeService.CreateCustomerPortalSession(r.Context(), orgID)
	if err != nil {
		http.Error(w, "failed to create portal session", http.StatusInternalServerError)
		return
	}

	writeJSON(w, CheckoutResponse{URL: url})
}
