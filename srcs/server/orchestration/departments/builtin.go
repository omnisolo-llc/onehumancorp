package departments

import (
	"context"
)

type BuiltinDepartment struct {
	Name string
}

func (b *BuiltinDepartment) HandleEvent(ctx context.Context, tenantID, eventType string, payload []byte) error {
	return nil
}

func (b *BuiltinDepartment) RetrieveMemoryContext(ctx context.Context, tenantID, query string, limit int) ([]string, error) {
	return []string{}, nil
}

func (b *BuiltinDepartment) EmitDraftAction(ctx context.Context, tenantID, actionType string, details []byte) error {
	return nil
}
