package departments

import (
	"context"
	"fmt"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"github.com/onehumancorp/mono/src/server/orchestration"
)

var tracer = otel.Tracer("onehumancorp/mono/src/server/domain/departments")

// BaseDepartment provides common functionality for departments.
type BaseDepartment struct {
	id         string
	deptType   DepartmentType
	hub        HubPublisher
	memory     MemoryLayer
	reviewCtr  ReviewCenterLayer
	agentID    string
}

func NewBaseDepartment(id string, deptType DepartmentType, memory MemoryLayer, reviewCtr ReviewCenterLayer, agentID string) *BaseDepartment {
	return &BaseDepartment{
		id:        id,
		deptType:  deptType,
		memory:    memory,
		reviewCtr: reviewCtr,
		agentID:   agentID,
	}
}

func (b *BaseDepartment) ID() string {
	return b.id
}

func (b *BaseDepartment) Type() DepartmentType {
	return b.deptType
}

func (b *BaseDepartment) HandleEvent(ctx context.Context, event orchestration.Message) error {
	// Default: do nothing
	return nil
}

func (b *BaseDepartment) RetrieveMemoryContext(ctx context.Context, query string) (*MemoryContext, error) {
	ctx, span := tracer.Start(ctx, "RetrieveMemoryContext")
	defer span.End()

	span.SetAttributes(attribute.String("query", query))
	return b.memory.Retrieve(ctx, query)
}

func (b *BaseDepartment) EmitDraftAction(ctx context.Context, action DraftAction) error {
	ctx, span := tracer.Start(ctx, "EmitDraftAction")
	defer span.End()

	span.SetAttributes(attribute.String("action_type", action.ActionType))
	return b.reviewCtr.ReceiveDraft(ctx, action)
}

func (b *BaseDepartment) Start(ctx context.Context, hub HubPublisher) error {
	b.hub = hub
	return nil
}

// OperationsDepartment handles orders.
type OperationsDepartment struct {
	*BaseDepartment
}

func (d *OperationsDepartment) HandleEvent(ctx context.Context, event orchestration.Message) error {
	if event.Type == "order.created" {
		ctx, span := tracer.Start(ctx, "OperationsDepartment.HandleEvent")
		defer span.End()

		span.SetAttributes(attribute.String("event_id", event.ID))

		err := d.memory.SaveEvent(ctx, event)
		if err != nil {
			return err
		}

		// Emit processed event
		if d.hub != nil {
			err = d.hub.Publish(orchestration.Message{
				ID:         fmt.Sprintf("evt-processed-%s", event.ID),
				FromAgent:  d.agentID,
				Type:       "order.processed",
				Content:    fmt.Sprintf("Order %s processed", event.ID),
				OccurredAt: time.Now().UTC(),
			})
			if err != nil {
				return err
			}
		}
	}
	return nil
}

// CustomerSuccessDepartment handles customer communication drafts.
type CustomerSuccessDepartment struct {
	*BaseDepartment
}

func (d *CustomerSuccessDepartment) HandleEvent(ctx context.Context, event orchestration.Message) error {
	if event.Type == "order.processed" {
		ctx, span := tracer.Start(ctx, "CustomerSuccessDepartment.HandleEvent")
		defer span.End()

		span.SetAttributes(attribute.String("event_id", event.ID))

		memCtx, err := d.RetrieveMemoryContext(ctx, "Get customer details")
		if err != nil {
			return err
		}

		contextStr := ""
		if memCtx != nil {
			contextStr = memCtx.Context
		}

		draft := DraftAction{
			ID:             fmt.Sprintf("draft-%s", event.ID),
			DepartmentType: d.Type(),
			AgentID:        d.agentID,
			ActionType:     "send_confirmation_message",
			Payload:        map[string]string{"message": "Thank you for your order!"},
			Status:         "draft",
			Description:    fmt.Sprintf("Draft confirmation for order event %s. Context: %s", event.ID, contextStr),
		}

		return d.EmitDraftAction(ctx, draft)
	}
	return nil
}
