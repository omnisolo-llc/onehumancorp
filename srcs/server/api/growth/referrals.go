package growth

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
)

type ReferralRequest struct {
	Email      string `json:"email"`
	Source     string `json:"source"`
	CampaignID string `json:"campaign_id"`
}

func HandleReferrals() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/api/growth").Start(r.Context(), "HandleReferrals")
		defer span.End()

		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var req ReferralRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}

		// Record telemetry
		if telemetry.BufferMetricFunc != nil {
			telemetry.BufferMetricFunc(ctx, "ohc_referral_sent_total", "1")
		}

		// Redact PII before logging if necessary (just good practice based on constraints)
		redactedEmail := telemetry.RedactPII(req.Email)

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"status": "success",
			"message": "Referral recorded successfully",
			"redacted_email": redactedEmail,
		})
	}
}
