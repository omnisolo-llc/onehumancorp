package integrations

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// SPIFFEValidator provides zero-trust identity verification for agent communication.
type SPIFFEValidator struct {
	trustDomain string
	mode        string // "cloud" or "standalone"
}

// NewSPIFFEValidator creates a new SPIFFEValidator based on environment settings.
func NewSPIFFEValidator() *SPIFFEValidator {
	mode := "standalone"
	if os.Getenv("OHC_MULTITENANT") == "true" {
		mode = "cloud"
	}

	return &SPIFFEValidator{
		trustDomain: os.Getenv("SPIFFE_TRUST_DOMAIN"),
		mode:        mode,
	}
}

// ValidateSVID verifies a SPIFFE ID string.
// In cloud mode, it ensures the ID belongs to the configured trust domain.
// In standalone mode, it uses a simplified/mock verification.
func (v *SPIFFEValidator) ValidateSVID(ctx context.Context, spiffeID string) error {
	if spiffeID == "" {
		return errors.New("empty SPIFFE ID")
	}

	if v.mode == "standalone" {
		slog.Debug("standalone mode: skipping strict SPIFFE validation", "id", spiffeID)
		return nil
	}

	// Basic validation for cloud mode
	if !strings.HasPrefix(spiffeID, "spiffe://") {
		return errors.New("invalid SPIFFE ID format: must start with 'spiffe://'")
	}

	if v.trustDomain != "" {
		expectedPrefix := fmt.Sprintf("spiffe://%s/", v.trustDomain)
		if !strings.HasPrefix(spiffeID, expectedPrefix) {
			return fmt.Errorf("identity %q does not belong to trust domain %q", spiffeID, v.trustDomain)
		}
	}

	return nil
}

// VerifyAgentIdentity checks if the SVID matches the expected agent identity.
func (v *SPIFFEValidator) VerifyAgentIdentity(ctx context.Context, spiffeID string, agentID string) error {
	if err := v.ValidateSVID(ctx, spiffeID); err != nil {
		return err
	}

	// Standard OHC SPIFFE ID format: spiffe://<trust-domain>/agent/<agent-id>
	parts := strings.Split(spiffeID, "/")
	if len(parts) < 5 || parts[3] != "agent" {
		// If not in standard format, we might just check if agentID is present in the ID.
		if !strings.Contains(spiffeID, agentID) {
			return fmt.Errorf("SPIFFE ID %q does not match agent %q", spiffeID, agentID)
		}
		return nil
	}

	if parts[4] != agentID {
		return fmt.Errorf("identity mismatch: expected agent %q, got %q from SVID", agentID, parts[4])
	}

	return nil
}

// AuthorizedForMesh checks if the context contains a valid SPIFFE identity authorized for mesh access.
func AuthorizedForMesh(ctx context.Context, v *SPIFFEValidator) bool {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return false
	}

	// For now, if we have valid claims, we consider it authorized.
	// In the future, we could extract the SVID from the context and use the validator.
	return true
}
