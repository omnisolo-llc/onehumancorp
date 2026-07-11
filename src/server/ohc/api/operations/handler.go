package operations

import (
	"context"
    "fmt"
    "log"

	domain "mono/src/server/ohc/domain/operations"
)

// Handler serves as the gRPC or REST interface for the Operations Manager Agent Protocol.
type Handler struct {
	manager *domain.OperationsManager
}

func NewHandler(manager *domain.OperationsManager) *Handler {
	return &Handler{manager: manager}
}

// ApproveActionCard handles the API request when an owner approves an action card.
// This executes the underlying operation securely.
func (h *Handler) ApproveActionCard(ctx context.Context, tenantID, actionType string, payload map[string]interface{}) error {
	intent := domain.ActionIntent{
		TenantID:   tenantID,
		ActionType: actionType,
		Payload:    payload,
	}

    err := h.manager.ExecuteAction(ctx, intent)
    if err != nil {
        log.Printf("Failed to execute action card: %v", err)
        return fmt.Errorf("failed to execute action card: %w", err)
    }

	return nil
}
