package orchestration

import (
	"context"
	"fmt"
	"github.com/google/uuid"
)

// BaseDepartment provides a basic implementation for memory and draft actions.
type BaseDepartment struct {
	name      string
	drafts    []DraftAction
	onEmit    func(DraftAction)
}

func (b *BaseDepartment) Name() string {
	return b.name
}

func (b *BaseDepartment) RetrieveMemoryContext(ctx context.Context, query string) (string, error) {
	// In a real implementation, this would query pgvector.
	return "Mocked pgvector context for: " + query, nil
}

func (b *BaseDepartment) EmitDraftAction(ctx context.Context, action DraftAction) error {
	action.ID = uuid.NewString()
	action.Status = "pending"
	action.Department = b.name
	b.drafts = append(b.drafts, action)
	if b.onEmit != nil {
		b.onEmit(action)
	}
	return nil
}

// OperationsDepartment
type OperationsDepartment struct {
	*BaseDepartment
}

func NewOperationsDepartment(onEmit func(DraftAction)) *OperationsDepartment {
	return &OperationsDepartment{
		BaseDepartment: &BaseDepartment{name: "Operations", onEmit: onEmit},
	}
}

func (d *OperationsDepartment) HandleEvent(ctx context.Context, event DepartmentEvent) error {
	if event.Type == "order.created" {
		// Example: Process the order internally
		fmt.Printf("Operations processed order: %v\n", event.Payload)
	}
	return nil
}

// MarketingDepartment
type MarketingDepartment struct {
	*BaseDepartment
}

func NewMarketingDepartment(onEmit func(DraftAction)) *MarketingDepartment {
	return &MarketingDepartment{
		BaseDepartment: &BaseDepartment{name: "Marketing", onEmit: onEmit},
	}
}

func (d *MarketingDepartment) HandleEvent(ctx context.Context, event DepartmentEvent) error {
	return nil
}

// SalesDepartment
type SalesDepartment struct {
	*BaseDepartment
}

func NewSalesDepartment(onEmit func(DraftAction)) *SalesDepartment {
	return &SalesDepartment{
		BaseDepartment: &BaseDepartment{name: "Sales", onEmit: onEmit},
	}
}

func (d *SalesDepartment) HandleEvent(ctx context.Context, event DepartmentEvent) error {
	return nil
}

// CustomerSuccessDepartment
type CustomerSuccessDepartment struct {
	*BaseDepartment
}

func NewCustomerSuccessDepartment(onEmit func(DraftAction)) *CustomerSuccessDepartment {
	return &CustomerSuccessDepartment{
		BaseDepartment: &BaseDepartment{name: "Customer Success", onEmit: onEmit},
	}
}

func (d *CustomerSuccessDepartment) HandleEvent(ctx context.Context, event DepartmentEvent) error {
	if event.Type == "order.created" {
		// Draft a confirmation message
		action := DraftAction{
			Description: "Draft confirmation message for new order.",
			Payload: map[string]interface{}{
				"message": "Thank you for your order! We are processing it now.",
				"orderInfo": event.Payload,
			},
		}
		return d.EmitDraftAction(ctx, action)
	}
	return nil
}

// FinanceDepartment
type FinanceDepartment struct {
	*BaseDepartment
}

func NewFinanceDepartment(onEmit func(DraftAction)) *FinanceDepartment {
	return &FinanceDepartment{
		BaseDepartment: &BaseDepartment{name: "Finance", onEmit: onEmit},
	}
}

func (d *FinanceDepartment) HandleEvent(ctx context.Context, event DepartmentEvent) error {
	return nil
}

// LegalDepartment
type LegalDepartment struct {
	*BaseDepartment
}

func NewLegalDepartment(onEmit func(DraftAction)) *LegalDepartment {
	return &LegalDepartment{
		BaseDepartment: &BaseDepartment{name: "Legal", onEmit: onEmit},
	}
}

func (d *LegalDepartment) HandleEvent(ctx context.Context, event DepartmentEvent) error {
	return nil
}

// AdvisoryDepartment
type AdvisoryDepartment struct {
	*BaseDepartment
}

func NewAdvisoryDepartment(onEmit func(DraftAction)) *AdvisoryDepartment {
	return &AdvisoryDepartment{
		BaseDepartment: &BaseDepartment{name: "Advisory", onEmit: onEmit},
	}
}

func (d *AdvisoryDepartment) HandleEvent(ctx context.Context, event DepartmentEvent) error {
	return nil
}
