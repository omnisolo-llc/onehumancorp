package operations

import (
	"context"
	"fmt"
    "log"
)

// DBExecutor represents the minimum interface needed for operations
type DBExecutor interface {
    ExecContext(ctx context.Context, query string, args ...interface{}) (Result, error)
    BeginTx(ctx context.Context, opts interface{}) (Tx, error)
}

type Tx interface {
    ExecContext(ctx context.Context, query string, args ...interface{}) (Result, error)
    Commit() error
    Rollback() error
}

// Result represents the result of a database execution
type Result interface {
    RowsAffected() (int64, error)
}

// OperationsManager coordinates executing high-level tasks based on agent intent.
// It bridges the gap between Agent Feed suggestions and system state mutations.
type OperationsManager struct {
    db DBExecutor
}

func NewOperationsManager(db DBExecutor) *OperationsManager {
	return &OperationsManager{db: db}
}

// ActionIntent represents a parsed intent from the Agent Feed.
type ActionIntent struct {
	TenantID   string
	ActionType string
	Payload    map[string]interface{}
}

// ExecuteAction executes a confirmed action intent. This is the entrypoint
// after an owner hits "Approve" on an Action Card in the Agent Feed.
func (om *OperationsManager) ExecuteAction(ctx context.Context, intent ActionIntent) error {
    // Start a transaction to ensure connection affinity for RLS
    tx, err := om.db.BeginTx(ctx, nil)
    if err != nil {
         return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback()

    // Setup RLS (Row Level Security) context for the tenant on this specific transaction connection
    _, err = tx.ExecContext(ctx, "SET LOCAL rls.tenant_id = $1", intent.TenantID)
    if err != nil {
        return fmt.Errorf("FATAL: failed to set RLS tenant_id %s: %w", intent.TenantID, err)
    }

	switch intent.ActionType {
	case "BOOKING_REQUEST":
		err = om.executeBookingRequest(ctx, tx, intent)
	case "INVENTORY_DEDUCTION":
		err = om.executeInventoryDeduction(ctx, tx, intent)
	default:
		err = fmt.Errorf("unsupported action type: %s", intent.ActionType)
	}

    if err != nil {
        return err
    }

    return tx.Commit()
}

func (om *OperationsManager) executeBookingRequest(ctx context.Context, tx Tx, intent ActionIntent) error {
	date, ok := intent.Payload["date"].(string)
    if !ok {
        return fmt.Errorf("invalid payload: missing date")
    }

    query := `INSERT INTO bookings (tenant_id, booking_date, status) VALUES ($1, $2, 'CONFIRMED')`

    result, err := tx.ExecContext(ctx, query, intent.TenantID, date)
    if err != nil {
        return fmt.Errorf("failed to execute booking: %w", err)
    }

    rows, err := result.RowsAffected()
    if err != nil {
         return fmt.Errorf("failed to read rows affected: %w", err)
    }

    log.Printf("Booking confirmed for %s on %s. Rows affected: %d", intent.TenantID, date, rows)

	return nil
}

func (om *OperationsManager) executeInventoryDeduction(ctx context.Context, tx Tx, intent ActionIntent) error {
	itemId, ok := intent.Payload["item_id"].(string)
    if !ok {
         return fmt.Errorf("invalid payload: missing item_id")
    }

    qty, ok := intent.Payload["quantity"].(float64)
    if !ok {
         return fmt.Errorf("invalid payload: missing quantity")
    }

    query := `UPDATE inventory SET quantity = quantity - $1 WHERE tenant_id = $2 AND item_id = $3 AND quantity >= $1`

    result, err := tx.ExecContext(ctx, query, qty, intent.TenantID, itemId)
    if err != nil {
        return fmt.Errorf("failed to deduct inventory: %w", err)
    }

    rows, err := result.RowsAffected()
    if err != nil {
         return fmt.Errorf("failed to read rows affected: %w", err)
    }

    if rows == 0 {
         return fmt.Errorf("insufficient inventory or item not found")
    }

    log.Printf("Inventory deducted for %s, item %s, qty %f", intent.TenantID, itemId, qty)
	return nil
}
