package orchestration

import (
	"context"
	"fmt"
	"sync"
)

type DepartmentManager struct {
	departments  map[string]Department
	draftActions map[string]*DraftAction
	mu           sync.RWMutex
}

func NewDepartmentManager() *DepartmentManager {
	dm := &DepartmentManager{
		departments:  make(map[string]Department),
		draftActions: make(map[string]*DraftAction),
	}

	onEmit := func(action DraftAction) {
		dm.mu.Lock()
		defer dm.mu.Unlock()
		dm.draftActions[action.ID] = &action
		fmt.Printf("Draft action emitted: %v\n", action)
	}

	dm.Register(NewOperationsDepartment(onEmit))
	dm.Register(NewMarketingDepartment(onEmit))
	dm.Register(NewSalesDepartment(onEmit))
	dm.Register(NewCustomerSuccessDepartment(onEmit))
	dm.Register(NewFinanceDepartment(onEmit))
	dm.Register(NewLegalDepartment(onEmit))
	dm.Register(NewAdvisoryDepartment(onEmit))

	return dm
}

func (dm *DepartmentManager) Register(d Department) {
	dm.mu.Lock()
	defer dm.mu.Unlock()
	dm.departments[d.Name()] = d
}

func (dm *DepartmentManager) DispatchEvent(ctx context.Context, event DepartmentEvent) {
	dm.mu.RLock()
	deps := make([]Department, 0, len(dm.departments))
	for _, d := range dm.departments {
		deps = append(deps, d)
	}
	dm.mu.RUnlock()

	for _, d := range deps {
		// In a real system, you'd likely dispatch asynchronously
		err := d.HandleEvent(ctx, event)
		if err != nil {
			fmt.Printf("Error handling event in department %s: %v\n", d.Name(), err)
		}
	}
}

func (dm *DepartmentManager) GetDraftActions() []DraftAction {
	dm.mu.RLock()
	defer dm.mu.RUnlock()

	actions := make([]DraftAction, 0, len(dm.draftActions))
	for _, action := range dm.draftActions {
		actions = append(actions, *action)
	}
	return actions
}

func (dm *DepartmentManager) UpdateDraftActionStatus(id string, status string) error {
	dm.mu.Lock()
	defer dm.mu.Unlock()

	action, exists := dm.draftActions[id]
	if !exists {
		return fmt.Errorf("draft action not found")
	}
	action.Status = status
	return nil
}
