package api

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/domain"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	quotaMeter   = otel.Meter("github.com/onehumancorp/mono/srcs/server/api/quota")
	quotaChecks  metric.Int64Counter
	quotaIncrements metric.Int64Counter
)

func init() {
	var err error
	quotaChecks, err = quotaMeter.Int64Counter("quota_checks_total", metric.WithDescription("Total quota checks"))
	if err != nil {
		panic(err)
	}
	quotaIncrements, err = quotaMeter.Int64Counter("quota_increments_total", metric.WithDescription("Total quota increments"))
	if err != nil {
		panic(err)
	}
}

type QuotaHandler struct {
	svc *domain.QuotaService
}

func NewQuotaHandler(svc *domain.QuotaService) *QuotaHandler {
	return &QuotaHandler{svc: svc}
}

func (h *QuotaHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodGet {
		h.handleGetQuota(w, r)
		return
	} else if r.Method == http.MethodPost {
		h.handleIncrementQuota(w, r)
		return
	}
	http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
}

func (h *QuotaHandler) handleGetQuota(w http.ResponseWriter, r *http.Request) {
	// Extract team ID from path: /api/quotas/{team_id}
	parts := strings.Split(r.URL.Path, "/")
	if len(parts) < 4 {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	teamID := parts[3]

	quotaChecks.Add(r.Context(), 1)

	status, err := h.svc.CheckQuota(r.Context(), teamID)
	if err == domain.ErrTeamNotFound {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	} else if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(status)
}

func (h *QuotaHandler) handleIncrementQuota(w http.ResponseWriter, r *http.Request) {
	// Extract team ID from path: /api/quotas/{team_id}/increment
	parts := strings.Split(r.URL.Path, "/")
	if len(parts) < 5 || parts[4] != "increment" {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	teamID := parts[3]

	quotaIncrements.Add(r.Context(), 1)

	err := h.svc.IncrementUsage(r.Context(), teamID)
	if err == domain.ErrTeamNotFound {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	} else if err == domain.ErrQuotaExceeded {
		http.Error(w, err.Error(), http.StatusTooManyRequests)
		return
	} else if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}
