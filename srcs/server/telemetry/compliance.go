package telemetry

import (
	"context"
	"strings"
	"fmt"
)

// AuditPayload represents a payload that might contain PII
type AuditPayload map[string]interface{}

// CheckForPIILock verifies that sensitive keys are not included using exact or word boundary matching
func CheckForPIILock(payload AuditPayload) error {
	sensitiveKeys := []string{
		"password", "secret", "key", "token", "auth", "cookie", "credential",
		"email", "phone", "ssn", "address", "pii", "tenant_id",
		"organization_id", "session_id", "payload", "credit_card", "cvv",
		"dob", "birth_date", "passport", "bank_account", "stripe", "billing",
		"ip_address", "mac_address", "geolocation",
	}

	for k := range payload {
		kLower := strings.ToLower(k)
		for _, sensitive := range sensitiveKeys {
			if kLower == sensitive || strings.Contains(kLower, "_"+sensitive) || strings.Contains(kLower, sensitive+"_") {
				return fmt.Errorf("PII Leakage Blocked: payload contains restricted key '%s'", k)
			}
		}
	}
	return nil
}

type contextKey string

const TenantContextKey contextKey = "tenant_id"

// CheckMultiTenantContext enforces tenant isolation logging rules
func CheckMultiTenantContext(ctx context.Context, tenantID string) error {
	if ctxTenant, ok := ctx.Value(TenantContextKey).(string); ok {
		if ctxTenant != "" && ctxTenant != tenantID {
			return fmt.Errorf("Cross-tenant leakage prevented: Context tenant %s does not match expected %s", ctxTenant, tenantID)
		}
	}
	return nil
}
