package autodream

import (
	"context"
	"encoding/json"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/orchestration/queue"
)

type PrunePayload struct {
	OrganizationID string `json:"organization_id"`
	StaleAgeHours  int    `json:"stale_age_hours"`
}

type ResolvePayload struct {
	OrganizationID string `json:"organization_id"`
}

// PruneJobHandler returns a job handler for pruning stale context in the background.
func PruneJobHandler(service *Service) queue.JobHandler {
	return func(ctx context.Context, job *queue.Job) error {
		var payload PrunePayload
		if err := json.Unmarshal([]byte(job.Payload), &payload); err != nil {
			return err
		}

		claims := &auth.Claims{OrganizationID: payload.OrganizationID}
		jobCtx := auth.ContextWithClaims(ctx, claims)

		staleAge := time.Duration(payload.StaleAgeHours) * time.Hour
		if staleAge == 0 {
			staleAge = 24 * time.Hour // Default 24h
		}

		if err := service.PruneStaleContext(jobCtx, payload.OrganizationID, staleAge); err != nil {
			slog.Error("Failed to prune stale context", "organization_id", payload.OrganizationID, "error", err)
			return err
		}

		slog.Info("Successfully pruned stale context", "organization_id", payload.OrganizationID)
		return nil
	}
}

// ResolveJobHandler returns a job handler for resolving memory conflicts in the background.
func ResolveJobHandler(service *Service) queue.JobHandler {
	return func(ctx context.Context, job *queue.Job) error {
		var payload ResolvePayload
		if err := json.Unmarshal([]byte(job.Payload), &payload); err != nil {
			return err
		}

		claims := &auth.Claims{OrganizationID: payload.OrganizationID}
		jobCtx := auth.ContextWithClaims(ctx, claims)

		if err := service.ResolveConflicts(jobCtx, payload.OrganizationID); err != nil {
			slog.Error("Failed to resolve memory conflicts", "organization_id", payload.OrganizationID, "error", err)
			return err
		}

		slog.Info("Successfully resolved memory conflicts", "organization_id", payload.OrganizationID)
		return nil
	}
}
