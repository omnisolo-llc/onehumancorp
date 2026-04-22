package telemetry

import (
	"context"
	"encoding/json"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

// ViolationStore tracks sandbox violations.
type ViolationStore struct {
	dbProvider      db.Provider
	violationCounter metric.Int64Counter
}

// NewViolationStore creates a new ViolationStore.
func NewViolationStore(dbProvider db.Provider, meter metric.Meter) (*ViolationStore, error) {
	counter, err := meter.Int64Counter(
		"ohc_agent_violations_total",
		metric.WithDescription("Total number of sandbox violations"),
	)
	if err != nil {
		return nil, err
	}

	return &ViolationStore{
		dbProvider:       dbProvider,
		violationCounter: counter,
	}, nil
}

// RecordViolation logs a sandbox violation to the DB and increments the metric.
func (s *ViolationStore) RecordViolation(ctx context.Context, agentID, sessionID, violationType, details string) error {
	id := uuid.New().String()
	tenantID := auth.OrganizationIDFromContext(ctx)

	detailsJSON, err := json.Marshal(map[string]string{"error": details})
	if err != nil {
		return err
	}

	query := `
		INSERT INTO agent_violations (id, tenant_id, agent_id, session_id, violation_type, details)
		VALUES ($1, $2, $3, $4, $5, $6)
	`

	_, err = s.dbProvider.Exec(ctx, query, id, tenantID, agentID, sessionID, violationType, string(detailsJSON))
	if err != nil {
		return err
	}

	s.violationCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("type", violationType)))

	return nil
}
